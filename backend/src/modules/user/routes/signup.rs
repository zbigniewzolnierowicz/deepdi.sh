use actix_web::{web, HttpResponse};
use common::user::{CreateNewUserDTO, UserDataDTO};
use eyre::Context;
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::user::{CreateNewUser, SignupError};

#[instrument(name = "Create a new account", skip(db, body))]
pub async fn create_account(
    db: web::Data<PgPool>,
    body: web::Json<CreateNewUserDTO>,
) -> Result<HttpResponse, SignupError> {
    let user = CreateNewUser::new(&body.username, &body.password, &body.email)
        .map_err(SignupError::Validation)?;

    insert_user(&db, &user).await.wrap_err("Database error")?;

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
