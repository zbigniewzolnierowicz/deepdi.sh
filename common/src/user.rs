#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateNewUserDTO<'a> {
    username: &'a str,
    password: &'a str,
    email: &'a str
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserDataDTO<'a> {
    username: &'a str,
    email: &'a str
}
