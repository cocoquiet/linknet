use std::fmt;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Url {
    value: String,
    domain: String,
}

impl Url {
    pub fn new(url: &str) -> Self {
        Url {
            value: url.to_string(),
            domain: get_domain(url).unwrap_or_default(),
        }
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Url {{ value: {}, domain: {} }}",
            self.value, self.domain
        )
    }
}

pub fn validate_url(url: &str) -> bool {
    let re =
        Regex::new(r"^https?://(?:www\.)?[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)+(?:/[^\s]*)?$").unwrap();
    re.is_match(url)
}

pub fn get_domain(url: &str) -> Option<String> {
    let re = Regex::new(r"^https?://(?:www\.)?([a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)+)").unwrap();
    if let Some(captures) = re.captures(url) {
        if let Some(domain) = captures.get(1) {
            return Some(domain.as_str().to_string());
        }
    }
    None
}
