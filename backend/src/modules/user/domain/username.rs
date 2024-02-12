use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Username(String);

impl Username {
    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
