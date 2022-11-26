extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_soundcloud_url(query: &str) -> String {
    if query == "sc" {
        let soundcloud_url = "https://soundcloud.com/feed";
        return soundcloud_url.to_string();
    }

    let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
    let soundcloud_url = format!("https://soundcloud.com/search?q={}", encoded_query);

    soundcloud_url
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_soundcloud_url() {
        let fake_query = "sc";
        assert_eq!(
            construct_soundcloud_url(fake_query),
            "https://soundcloud.com/feed"
        );
    }

    #[test]
    fn test_construct_soundcloud_url_query() {
        let fake_query = "sc test";
        assert_eq!(
            construct_soundcloud_url(fake_query),
            "https://soundcloud.com/search?q=test"
        );
    }
}
