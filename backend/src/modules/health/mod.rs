use std::collections::HashMap;
use std::pin::Pin;

use actix_web::{http::StatusCode, web, HttpResponse};
use futures::{self, future};
use sqlx::PgPool;
use std::future::Future;
use tracing::instrument;

#[derive(serde::Serialize)]
struct HealthChecks {
    checks: HashMap<&'static str, String>,
}

#[instrument(name = "Health check", skip(db, redis))]
pub async fn health_check(db: web::Data<PgPool>, redis: web::Data<redis::Client>) -> HttpResponse {
    type HealthCheckResult = Result<(), anyhow::Error>;
    type HealthCheckEntry = (&'static str, HealthCheckResult);
    type StatusCheck = Pin<Box<dyn Future<Output = HealthCheckEntry>>>;

    let redis_check = async move { ("redis", redis_status(&redis).await) };
    let db_check = async move { ("db", db_status(&db).await) };

    let futures: Vec<StatusCheck> = vec![Box::pin(redis_check), Box::pin(db_check)];
    let status_checks_futures = future::join_all(futures).await;

    let status: StatusCode = if status_checks_futures.iter().any(|c| c.1.is_err()) {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    };

    let checks: HashMap<&str, String> =
        HashMap::from_iter(status_checks_futures.into_iter().map(|(key, value)| {
            (
                key,
                value.map_or_else(|error| error.to_string(), |_| "OK!".to_string()),
            )
        }));

    HttpResponse::build(status).json(HealthChecks { checks })
}

#[tracing::instrument(name = "Database health check", skip(db))]
async fn db_status(db: &PgPool) -> Result<(), anyhow::Error> {
    sqlx::query("SELECT 1;").execute(db).await?;

    Ok(())
}

#[tracing::instrument(name = "Redis health check", skip(conn))]
async fn redis_status(conn: &redis::Client) -> Result<(), anyhow::Error> {
    let mut conn = conn.get_connection()?;
    redis::cmd("PING").query(&mut conn)?;
    Ok(())
}
