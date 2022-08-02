use std::{env, process};

use regex::Regex;

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
        _ => {
            match command.parse() {
                Ok(argument) => print_page(argument),
                _ => {
                    eprintln!("error: could not parse argument.");
                    process::exit(1);
                }
            }
        }
    }
}

fn print_help() {
    println!("txtvrs, version {}", VERSION);
    println!("======================");
    println!("help\t\tsee this message");
    println!("<n>\t\tretrieve page n from SVT Text");
}

fn print_page(page: u32)
{
    let response = reqwest::blocking::get(
        "https://www.svt.se/text-tv/webb/".to_owned() + &page.to_string()
    )
        .expect("Failed to get page")
        .text()
        .expect("Failed to retrieve text");

    let document = scraper::Html::parse_document(&response);
    let screen_selector = scraper::Selector::parse("div.TextContent_textWrapper__3s3Q0>div")
       .expect("Failed to parse headers");

    let screens = document.select(&screen_selector).map(
        |x| x.inner_html());

    let regex = Regex::new(r"<a[^>]+>(\d\d\d)</a>").unwrap();
    let screens = screens.map(
        |x| {regex.replace_all(x.as_str(), "$1").to_string()}
    );

    for text in screens {
        println!("{}", text);
    }
}
