use regex::Regex;

pub fn validate_url(url: &str) -> bool {
    let re =
        Regex::new(r"^https?://(?:www\.)?[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)+(?:/[^\s]*)?$").unwrap();
    re.is_match(url)
}
