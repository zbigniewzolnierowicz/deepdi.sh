use serde::{Deserialize, Serialize};

use crate::modules::user::domain::{Email, Username, HashedPassword};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: Username,
    pub password_hash: HashedPassword,
    pub email: Email,
}

impl User {
    pub fn new(username: &str, password_hash: &str, email: &str) -> Result<Self, String> {
        let username = Username::parse(username)?;
        let email = Email::parse(email)?;
        let password_hash = HashedPassword::parse(password_hash)?;

        Ok(Self {
            username,
            password_hash,
            email,
        })
    }
}

impl From<User> for common::user::UserDataDTO {
    fn from(val: User) -> Self {
        Self {
            email: val.email.as_ref().to_string(),
            username: val.username.as_ref().to_string(),
        }
    }
}
