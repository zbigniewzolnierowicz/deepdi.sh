use actix_web::{web, Responder};
use sqlx::PgPool;
use tracing::instrument;

#[instrument]
pub async fn create_account(db: web::Data<PgPool>) -> impl Responder {
    "WILL IMPLEMENT LATER"
}
