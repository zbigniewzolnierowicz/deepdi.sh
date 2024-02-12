#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateNewUserDTO {
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserDataDTO {
    pub username: String,
    pub email: String
}
