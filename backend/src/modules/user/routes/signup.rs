use actix_web::{web, HttpResponse};
use anyhow::Context;
use common::user::{CreateNewUserDTO, UserDataDTO};
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::user::errors::signup::SignupError;
use crate::modules::user::models::user::CreateNewUser;

#[instrument(name = "Create a new account", skip(db, body))]
pub async fn create_account(
    db: web::Data<PgPool>,
    body: web::Json<CreateNewUserDTO>,
) -> Result<HttpResponse, SignupError> {
    let user = CreateNewUser::new(&body.username, &body.password, &body.email)
        .map_err(SignupError::Validation)?;

    insert_user(&db, &user).await.context("Database error")?;

    Ok(HttpResponse::Ok().json(UserDataDTO::from(user)))
}

#[instrument(name = "Persist user in the database", skip(db, user))]
async fn insert_user(db: &PgPool, user: &CreateNewUser) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (username, password_hash, email) VALUES ($1, $2, $3)",
        user.username.as_ref(),
        user.password_hash.as_ref(),
        user.email.as_ref()
    )
    .execute(db)
    .await?;

    Ok(())
}
