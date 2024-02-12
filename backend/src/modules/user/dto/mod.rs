use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    id: uuid::Uuid,
    username: &'a str,
    password_hash: &'a str,
    email: &'a str,
}
