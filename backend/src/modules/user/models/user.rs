use serde::{Deserialize, Serialize};

use crate::modules::user::domain::{Email, Username};

#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    id: uuid::Uuid,
    username: Username,
    password_hash: &'a str,
    email: Email,
}
