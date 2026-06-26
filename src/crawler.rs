use crate::url::Url;
use reqwest::blocking::get;
use scraper::{Html, Selector};

pub fn crawl(url: &Url) {
    match get(url.value()) {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().unwrap();
                let document = Html::parse_document(&body);
                let selector = Selector::parse("a").unwrap();

                for element in document.select(&selector) {
                    if let Some(a) = element.value().attr("href") {
                        println!("Found link: {}", Url::new(a));
                    }
                }
            } else {
                eprintln!(
                    "Failed to fetch URL: {}. Status: {}",
                    url,
                    response.status()
                );
            }
        }
        Err(e) => {
            eprintln!("Error fetching URL: {}. Error: {}", url, e);
        }
    }
}
