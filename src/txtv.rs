use regex::Regex;

pub fn print_page(page: u32)
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