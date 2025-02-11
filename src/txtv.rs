enum PageError {
    Connection,
    Empty,
}

enum PageFormat {
    FirstPage,
    Article,
    Other,
}

pub fn print_page(page: u32) {
    use PageError::{Connection, Empty};
    let page_content = get_page_content(page);

    match page_content {
        Ok(content) => {
            for text in content {
                println!("{}", text);
            }
        }
        Err(why) => match why {
            Empty => println!("no page at {}", page),
            Connection => println!("connection error"),
        },
    }
}

fn get_page_format(page: u32) -> PageFormat {
    match page {
        100 => PageFormat::FirstPage,
        _ => PageFormat::Other,
    }
}

fn get_page_content(page: u32) -> Result<Vec<String>, PageError> {
    use PageError::{Connection, Empty};

    let response =
        reqwest::blocking::get("https://www.svt.se/text-tv/".to_owned() + &page.to_string());

    if response.is_err() {
        return Err(Connection);
    }

    let response = response.unwrap().text();

    if response.is_err() {
        return Err(Empty);
    }

    let response = response.unwrap();
    let document = scraper::Html::parse_document(&response);
    let screen_selector = scraper::Selector::parse("div[class*=\"Content_screenreaderOnly__\"]");

    if screen_selector.is_err() {
        return Err(Empty);
    }

    let screen_selector = screen_selector.unwrap();

    let screens = document.select(&screen_selector);

    let screens = screens.map(|x| {
        x.inner_html()
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&amp;", "&")
            .replace("&#x27;", "'")
    });

    let content: Vec<String> = screens.collect();
    if content.len() > 0 {
        Ok(content)
    } else {
        Err(Empty)
    }
}
