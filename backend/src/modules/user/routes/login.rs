use actix_session::Session;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use common::user::UserDataDTO;
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::user::{CreateNewUser, Email, HashedPassword, LoginError, Username};

#[instrument(name = "User logs in", skip(db, body, session))]
pub async fn log_in(
    db: web::Data<PgPool>,
    body: web::Json<common::user::LoginUserDTO>,
    session: Session,
) -> Result<HttpResponse, LoginError> {
    let (id, user) = find_user(&db, &body.username).await?;
    if !user.check_password(&body.password) {
        return Err(LoginError::WrongPassword);
    };

    persist_id_in_session(id, &session).await?;

    Ok(HttpResponse::Ok().json(UserDataDTO::from(user)))
}

#[instrument(name = "Persist user ID in session", skip(session))]
async fn persist_id_in_session(id: i32, session: &Session) -> Result<(), LoginError> {
    session
        .insert("user_id", id)
        .context("Could not insert into session")?;
    Ok(())
}

#[instrument(name = "Find user in database", skip(db, username))]
async fn find_user(db: &PgPool, username: &str) -> Result<(i32, CreateNewUser), LoginError> {
    let user = sqlx::query!(
        "SELECT id, username, email, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(db)
    .await
    .context("Database error")?
    .ok_or(LoginError::NotFound)?;
    let id = user.id;

    let user = CreateNewUser {
        username: Username::new(&user.username),
        email: Email::new(&user.email),
        password_hash: HashedPassword::new(&user.password_hash),
    };

    Ok((id, user))
}
