#![allow(unused)]
use validator::ValidateEmail;
#[derive(Debug)]
pub struct SubscriberEmail(String);


impl AsRef<str> for SubscriberEmail {
  fn as_ref(&self) -> &str {
      &self.0
  }
}

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<SubscriberEmail, String> {
      if email.validate_email() {
        Ok(Self(email))
      } else {
        Err(format!("{} is not a valid subscriber email", email))
      }
    }
}