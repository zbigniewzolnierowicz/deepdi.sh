use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Username(pub String);

impl Username {
    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(Self(s.to_string()))
    }

    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
