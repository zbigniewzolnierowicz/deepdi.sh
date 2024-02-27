use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct CreateNewUserDTO {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct LoginUserDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, TS)]
#[ts(export)]
pub struct UserDataDTO {
    pub username: String,
    pub email: String,
}
