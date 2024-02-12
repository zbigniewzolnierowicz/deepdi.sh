use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;
use validator::validate_email;

#[derive(Shrinkwrap, Serialize, Deserialize, Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty = s.trim().is_empty();
        let valid_email = validate_email(s.clone());

        if is_empty || !valid_email {
            Err(format!("{s} is not a valid email"))
        } else {
            Ok(Self(s))
        }
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
