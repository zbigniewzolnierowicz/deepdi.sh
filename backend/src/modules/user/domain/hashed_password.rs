use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn parse(s: &str) -> Result<Self, String> {
        let hashed_password = bcrypt::hash(s, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;

        Ok(Self(hashed_password))
    }

    pub fn new(hashed_password: &str) -> Self {
        Self(hashed_password.to_string())
    }
}

impl std::fmt::Display for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
