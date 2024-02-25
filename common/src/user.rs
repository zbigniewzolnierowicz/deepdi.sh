use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateNewUserDTO {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginUserDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserDataDTO {
    pub username: String,
    pub email: String,
}
