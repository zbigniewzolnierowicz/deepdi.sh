use actix_web::{web, HttpResponse};
use anyhow::Context;
use common::user::UserDataDTO;
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::user::domain::{Email, HashedPassword, Username};
use crate::modules::user::errors::login::LoginError;
use crate::modules::user::models::user::User;

#[instrument(name = "User logs in", skip(db, body))]
pub async fn log_in(
    db: web::Data<PgPool>,
    body: web::Json<common::user::LoginUserDTO>,
) -> Result<HttpResponse, LoginError> {
    let (id, user) = find_user(&db, &body.username).await?;
    if !user.check_password(&body.password) {
        return Err(LoginError::WrongPassword);
    };

    tracing::info!(user_id = id);

    Ok(HttpResponse::Ok().json(UserDataDTO::from(user)))
}

#[instrument(name = "Find user in database", skip(db, username))]
async fn find_user(db: &PgPool, username: &str) -> Result<(i32, User), LoginError> {
    let user = sqlx::query!(
        "SELECT id, username, email, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(db)
    .await
    .context("Database error")?
    .ok_or(LoginError::NotFound)?;
    let id = user.id;

    let user = User {
        username: Username::new(&user.username),
        email: Email::new(&user.email),
        password_hash: HashedPassword::new(&user.password_hash),
    };

    Ok((id, user))
}
