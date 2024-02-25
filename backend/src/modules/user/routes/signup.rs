use actix_web::{web, HttpResponse};
use common::user::{CreateNewUserDTO, UserDataDTO};
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::user::{CreateNewUser, SignupError};

#[instrument(name = "Create a new account", skip(db, body))]
#[utoipa::path(
    post,
    path = "/user/signup",
    request_body = CreateNewUserDTO,
    responses(
        (status = 201, description = "User created", body = UserDataDTO),
        (status = 409, description = "The user already exists", body = ErrorMessageWithJsonValue),
        (status = 500, description = "Fatal error", body = ErrorMessageWithJsonValue)
    )
)]
/// Endpoint for creating a new account
pub async fn create_account(
    db: web::Data<PgPool>,
    body: web::Json<CreateNewUserDTO>,
) -> Result<HttpResponse, SignupError> {
    let user = CreateNewUser::new(&body.username, &body.password, &body.email)
        .map_err(SignupError::Validation)?;

    insert_user(&db, &user).await?;

    Ok(HttpResponse::Created().json(UserDataDTO::from(user)))
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
