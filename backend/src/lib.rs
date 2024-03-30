pub mod configuration;
mod domain;

use std::collections::HashMap;

use axum::{extract::State, routing::get, Json, Router};
use serde_json::Value;
use sqlx::{
    postgres::PgConnectOptions,
    PgPool,
};

pub struct App {
    router: Router,
}

async fn health(State(db): State<PgPool>) -> Json<Value> {
    let db_ok = sqlx::query("SELECT 1;").execute(&db).await.is_ok();
    let details: HashMap<&'static str, bool> = HashMap::from([("database", db_ok)]);

    let health = details.values().into_iter().all(|check| !!check);
    Json(serde_json::json!({
        "health": health,
        "details": details
    }))
}

impl App {
    pub async fn new(db_settings: PgConnectOptions) -> color_eyre::Result<Self> {
        let db = PgPool::connect_lazy_with(db_settings);
        let router = Router::new()
            .route("/", get(|| async { "Hello, world!" }))
            .route("/healthz", get(health))
            .with_state(db);

        Ok(App { router })
    }

    pub async fn serve(self, listener: tokio::net::TcpListener) -> color_eyre::Result<()> {
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
