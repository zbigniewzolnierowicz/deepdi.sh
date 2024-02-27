use actix_session::Session;
use actix_web::{web, HttpResponse};
use eyre::Context;
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::user::{errors::get::GetUserDataError, UserData};

#[instrument(name = "Get user data", skip(db, session))]
pub async fn get_user_data(
    db: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, GetUserDataError> {
    let user_id = session
        .get::<i32>("user_id")
        .wrap_err("Session get")?
        .ok_or(GetUserDataError::NotFound)?;
    let user: common::user::UserDataDTO =
        find_user(&db, &user_id).await.wrap_err("Database")?.into();

    Ok(HttpResponse::Ok().json(user))
}

#[instrument(name = "Find user in database", skip(db))]
async fn find_user(db: &PgPool, id: &i32) -> Result<UserData, sqlx::Error> {
    sqlx::query_as!(
        UserData,
        "SELECT id, username, email FROM users WHERE id = $1",
        id
    )
    .fetch_one(db)
    .await
}
