use serde::{Deserialize, Serialize};
use validator::validate_email;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, String> {
        let is_empty = s.trim().is_empty();
        let valid_email = validate_email(s);

        if is_empty || !valid_email {
            Err(format!("{s} is not a valid email"))
        } else {
            Ok(Self(s.to_string()))
        }
    }

    pub fn new(email: &str) -> Self {
        Self(email.to_string())
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
