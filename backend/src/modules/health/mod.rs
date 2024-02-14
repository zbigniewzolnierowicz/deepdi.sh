use std::collections::HashMap;
use std::pin::Pin;

use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use futures;
use sqlx::PgPool;
use std::future::Future;
use tracing::instrument;

#[derive(serde::Serialize, serde::Deserialize)]
struct HealthChecks {
    checks: HashMap<String, String>,
}

#[instrument(name = "Health check", skip(db))]
pub async fn health_check(db: web::Data<PgPool>) -> HttpResponse {
    type StatusCheck = Pin<Box<dyn Future<Output = (&'static str, Result<(), anyhow::Error>)>>>;

    let redis_check = async {
        (
            "redis",
            Err(anyhow::anyhow!("Redis error"))
            // redis_status()
            //     .await
            //     .map_err(|_| anyhow::anyhow!("Redis error")),
        )
    };
    let db_check = async move { ("db", db_status(&db).await.context("Database error")) };
    let futures: [StatusCheck; 2] = [Box::pin(redis_check), Box::pin(db_check)];
    let status_checks_futures = futures::future::join_all(futures).await;

    let status: StatusCode = if status_checks_futures.iter().any(|c| c.1.is_err()) {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    };

    let checks: HashMap<String, String> =
        HashMap::from_iter(status_checks_futures.into_iter().map(|(key, value)| {
            (
                key.to_string(),
                value.map_or_else(|error| error.to_string(), |_| "OK!".to_string()),
            )
        }));

    HttpResponse::build(status).json(HealthChecks { checks })
}

#[tracing::instrument(name = "Check status of database", skip(db))]
async fn db_status(db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1;").execute(db).await?;

    Ok(())
}

async fn redis_status() -> Result<(), ()> {
    Ok(())
}
