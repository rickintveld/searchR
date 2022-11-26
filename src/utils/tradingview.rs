extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_tradingview_url(query: &str) -> String {
    if query == "tv" {
        let url = "https://tradingview.com";
        return url.to_string();
    }

    let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
    let url = format!("https://tradingview.com/symbols/{}", encoded_query);

    url
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_soundcloud_url() {
        let fake_query = "tv";
        assert_eq!(
            construct_tradingview_url(fake_query),
            "https://tradingview.com"
        );
    }

    #[test]
    fn test_construct_soundcloud_url_query() {
        let fake_query = "tv EURUSD";
        assert_eq!(
            construct_tradingview_url(fake_query),
            "https://tradingview.com/symbols/EURUSD"
        );
    }
}
