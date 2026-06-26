use std::env;
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

    crawler::crawl(&url_node);
}
