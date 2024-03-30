pub mod configuration;

use std::time::Duration;

use axum::{extract::State, routing::get, Router};
use serde::Serialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

pub struct App {
    router: Router,
}

async fn health(State(db): State<PgPool>) -> axum::response::Result<String> {
    let result = sqlx::query!("SELECT 1 as test;").fetch_one(&db).await.unwrap();
    Ok(result.test.unwrap().to_string())
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
