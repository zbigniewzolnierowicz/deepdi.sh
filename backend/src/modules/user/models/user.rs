use serde::{Deserialize, Serialize};

use crate::modules::user::domain::{Email, HashedPassword, Username};

#[derive(Serialize, Deserialize)]
pub struct CreateNewUser {
    pub username: Username,
    pub password_hash: HashedPassword,
    pub email: Email,
}

impl CreateNewUser {
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

    pub fn check_password(&self, verification_password: &str) -> bool {
        bcrypt::verify(verification_password, self.password_hash.as_ref()).unwrap_or(false)
    }
}

impl From<CreateNewUser> for common::user::UserDataDTO {
    fn from(val: CreateNewUser) -> Self {
        Self {
            email: val.email.as_ref().to_string(),
            username: val.username.as_ref().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: Username,
    pub password_hash: HashedPassword,
    pub email: Email,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl From<UserData> for common::user::UserDataDTO {
    fn from(val: UserData) -> Self {
        Self {
            username: val.username,
            email: val.email,
        }
    }
}
