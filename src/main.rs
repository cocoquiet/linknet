use std::{
    collections::{HashSet, VecDeque},
    env,
};
mod crawler;
mod url;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    let url = args.into_iter().nth(1).unwrap();
    if !url::validate_url(&url) {
        eprintln!("Invalid URL: {}", url);
        std::process::exit(1);
    }

    let url_node = url::Url::new(&url);
    println!("Valid URL: {}", url_node);

    let mut url_queue = VecDeque::new();
    let mut visited_urls = HashSet::new();
    url_queue.push_back(url_node.clone());
    visited_urls.insert(url_node);

    while !url_queue.is_empty() {
        let current_url = url_queue.pop_front().unwrap();
        println!("Crawling URL: {}", current_url);

        for link in crawler::crawl(&current_url) {
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
