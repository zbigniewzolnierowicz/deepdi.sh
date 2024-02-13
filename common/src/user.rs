use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateNewUserDTO {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataDTO {
    pub username: String,
    pub email: String,
}
