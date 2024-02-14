mod errors;

use actix_web::web;
use sqlx::PgPool;
use tracing::instrument;

use self::errors::HealthCheckError;

#[instrument(name = "Health check", skip(db))]
pub async fn health_check(db: web::Data<PgPool>) -> Result<String, HealthCheckError> {
    db_status(&db).await?;

    // TODO: add Redis health check

    Ok("All good!".into())
}

#[tracing::instrument(name = "Check status of database", skip(db))]
async fn db_status(db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1;").execute(db).await?;

    Ok(())
}
