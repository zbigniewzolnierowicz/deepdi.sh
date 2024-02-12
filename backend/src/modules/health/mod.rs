mod errors;

use actix_web::{web, Scope};
use sqlx::PgPool;
use tracing::instrument;

use self::errors::HealthCheckError;

#[instrument(name = "Health check", skip(db))]
async fn health_check(db: web::Data<PgPool>) -> Result<String, HealthCheckError> {
    db_status(&db).await?;

    Ok("All good!".into())
}

#[tracing::instrument(name = "Check status of database", skip(db))]
async fn db_status(db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1;").execute(db).await?;

    Ok(())
}

pub fn router(base_route: &str) -> Scope {
    web::scope(base_route).route("", web::get().to(health_check))
}
