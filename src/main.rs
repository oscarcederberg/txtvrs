use std::{env, process};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("error: no arguments passed.");
            process::exit(1);
        }
        2 => print_page(&args[1]),
        _ => {
            eprintln!("error: too many arguments passed.");
            process::exit(1);
        }
    }
}

fn print_page(page: &String)
{
    let response = reqwest::blocking::get(
        "https://www.svt.se/text-tv/webb/".to_owned() + page
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
