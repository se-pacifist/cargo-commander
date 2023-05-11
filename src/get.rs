mod command;
mod script;
mod utils;

use command::Command;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[0].contains("cargo-cmd") && args[1] == "cmd".to_string() {
            args.remove(1);
        }
    }
    args.remove(0);

    let mut command_args: Vec<String> = vec![];
    let mut commander_args: HashMap<String, String> = HashMap::new();

    let mut all_found = false;
    while args.len() > 0 {
        if all_found {
            command_args.push(args.remove(0));
        } else {
            if args[0].starts_with("-") {
                if args[0] == "-f" || args[0] == "--file" {
                    args.remove(0);
                    commander_args.insert("file".to_string(), args.remove(0));
                } else if args[0] == "-h" || args[0] == "--help" {
                    println!(
                        r"cargo-commander 2.0.15
A tool for printing information from toml files

USAGE:
    cargo get [SPECIFIERS...] [<ARGUMENTS>...]

ARGS:
    SPECIFIERS           Entries to print, e.g 'package.version package.repository'
    <ARGUMENTS>...       Arguments to the command

OPTIONS:
    -h, --help           Print help information
    -f, --file PATH      Custom path to command file to parse"
                    );
                    return Ok(());
                } else {
                    all_found = true;
                }
            } else {
                all_found = true;
            }
        }
    }

    println!("{:?}", command_args);
    println!("{:?}", commander_args);

    Ok(())
}
