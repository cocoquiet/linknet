use std::collections::{HashSet, VecDeque};

use crate::url::Url;
use reqwest::blocking::get;
use scraper::{Html, Selector};

pub fn crawl_single(url: &Url) -> Vec<Url> {
    let mut links = Vec::new();

    match get(&url.value) {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().unwrap();
                let document = Html::parse_document(&body);
                let selector = Selector::parse("a").unwrap();

                for element in document.select(&selector) {
                    if let Some(a) = element.value().attr("href") {
                        links.push(Url::new(a));
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

    links
}

pub fn crawl_recursive(init: &Url) {
    let mut url_queue = VecDeque::new();
    let mut visited_urls = HashSet::new();
    url_queue.push_back(init.clone());
    visited_urls.insert(init.clone());

    while !url_queue.is_empty() {
        let current_url = url_queue.pop_front().unwrap();
        println!("Crawling URL: {}", current_url);

        for link in crawl_single(&current_url) {
            if !visited_urls.contains(&link) {
                visited_urls.insert(link.clone());

                if link.domain == current_url.domain {
                    url_queue.push_front(link);
                } else {
                    url_queue.push_back(link);
                }
            }
        }
    }
}
