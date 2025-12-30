#![allow(unused)]
use unicode_segmentation::UnicodeSegmentation;

use crate::domain::subscriber_email::SubscriberEmail;

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