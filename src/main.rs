mod txtv;

use std::{env, process};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("error: no arguments passed.");
            process::exit(1);
        }
        2 => parse_command(&args[1]),
        _ => {
            eprintln!("error: too many arguments passed.");
            process::exit(1);
        }
    }
}

fn parse_command(command: &String) {
    match command.to_lowercase().as_str() {
        "help" => print_help(),
        _ => match command.parse() {
            Ok(argument) => txtv::print_page(argument),
            _ => {
                eprintln!("error: could not parse argument.");
                process::exit(1);
            }
        },
    }
}

fn print_help() {
    println!("txtvrs, version {}", VERSION);
    println!("======================");
    println!("help\t\tsee this message");
    println!("<n>\t\tretrieve page n from SVT Text");
}
