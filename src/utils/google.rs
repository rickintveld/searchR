extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_google_search_url(query: &str) -> String {
    let encoded_query: String = utf8_percent_encode(query, FRAGMENT).to_string();
    let google_search_url: String = format!("https://www.google.com/search?q={}", encoded_query);

    google_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let test_query = "hello";

        assert_eq!(
            construct_google_search_url(test_query),
            "https://www.google.com/search?q=hello"
        )
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let test_query = "hello world";
        assert_eq!(
            construct_google_search_url(test_query),
            "https://www.google.com/search?q=hello%20world"
        )
    }
}
