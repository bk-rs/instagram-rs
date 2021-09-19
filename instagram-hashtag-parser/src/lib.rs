#[cfg(feature = "with-crate-hashtag")]
use hashtag::HashtagParser;

#[cfg(feature = "with-regex")]
use once_cell::sync::Lazy;
#[cfg(feature = "with-regex")]
use regex::Regex;

#[cfg(feature = "with-regex")]
// https://stackoverflow.com/a/31965629/918930
static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"#((\w|[\u00A1-\uFFFF])+)").unwrap());

#[cfg(feature = "with-crate-hashtag")]
pub fn hashtags_with_crate_hashtag(s: &str) -> Vec<String> {
    HashtagParser::new(s)
        .into_iter()
        .map(|tag| tag.text.to_string())
        .collect::<Vec<_>>()
}

#[cfg(feature = "with-regex")]
pub fn hashtags_with_regex(s: &str) -> Vec<String> {
    RE.captures_iter(s)
        .into_iter()
        .map(|x| x[1].to_string())
        .collect()
}

#[cfg(feature = "with-crate-hashtag")]
#[cfg(test)]
mod tests_with_crate_hashtag {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            hashtags_with_crate_hashtag("#rust is #awesome"),
            vec!["rust".to_owned(), "awesome".to_owned()]
        );

        assert_eq!(
            hashtags_with_crate_hashtag("#我#我的"),
            vec!["我".to_owned(), "我的".to_owned()]
        );
    }
}

#[cfg(feature = "with-regex")]
#[cfg(test)]
mod tests_with_regex {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            hashtags_with_regex("#rust is #awesome"),
            vec!["rust".to_owned(), "awesome".to_owned()]
        );

        assert_eq!(
            hashtags_with_regex("#我#我的"),
            vec!["我".to_owned(), "我的".to_owned()]
        );
    }
}
