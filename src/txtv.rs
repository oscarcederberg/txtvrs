pub fn print_page(page: u32) {
    let response =
        reqwest::blocking::get("https://www.svt.se/text-tv/".to_owned() + &page.to_string())
            .expect("Failed to get page")
            .text()
            .expect("Failed to retrieve text");

    let document = scraper::Html::parse_document(&response);
    let screen_selector = scraper::Selector::parse("div.Content_screenreaderOnly__Gwyfj")
        .expect("Failed to parse content");

    let screens = document.select(&screen_selector);
    
    let screens = screens.map(|x| {
        x.inner_html()
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&#x27;", "'")
    });

    for text in screens {
        println!("{}", text);
    }
}
