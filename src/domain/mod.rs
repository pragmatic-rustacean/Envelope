pub mod new_subscriber;
pub mod subscriber_email;
pub mod subscriber_name;

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};
    use tracing::Subscriber;

    use crate::domain::{
        subscriber_email::SubscriberEmail,
        subscriber_name::{NewSubscriber, SubscriberName},
    };

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

    #[test]
    fn empty_string_are_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_without_at_symbol_is_rejected() {
        let email = "eyesonly72gmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@gmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
