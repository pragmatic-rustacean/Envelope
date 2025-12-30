
#![allow(unused)]
use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: String,
}

#[derive(Debug)]
pub struct SubscriberName(pub String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn parse_name(name: String) -> Result<Self, String> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_long = name.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbidden_characters =
            name.chars().any(|ch| forbidden_characters.contains(&ch));

        if is_long || contains_forbidden_characters || is_empty_or_whitespace {
            Err(format!("{} is not a valid name", name))
        } else {
            Ok(Self(name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::parse_name(name));
    }
    #[test]
    fn a_name_longer_than_256_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse_name(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = "  ".to_string();
        assert_err!(SubscriberName::parse_name(name));
    }

    #[test]
    fn name_containing_an_invalid_character_is_rejected() {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        for name in &forbidden_characters {
          let name = name.to_string();
          assert_err!(SubscriberName::parse_name(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfuly() {
      let name = "James Muriuki Maina".to_string();
      assert_ok!(SubscriberName::parse_name(name));
    }
}
