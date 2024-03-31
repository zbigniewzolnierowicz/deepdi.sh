mod routes;

use std::sync::Arc;

use crate::domain::repositories::ingredients::{
    InMemoryIngredientRepository, IngredientRepository,
};
use axum::{routing::post, Router};
use sqlx::postgres::PgConnectOptions;

use self::routes::create_ingredient::create_ingredient_route;

pub struct App {
    router: Router,
}

#[derive(Clone)]
pub struct AppState {
    pub ingredient_repository: Arc<dyn IngredientRepository>,
}

impl App {
    pub async fn new(_db_settings: PgConnectOptions) -> color_eyre::Result<Self> {
        // let db = PgPool::connect_lazy_with(db_settings);
        let ingredient_repository = Arc::new(InMemoryIngredientRepository::new());
        let state = AppState {
            ingredient_repository,
        };
        let router = Router::new()
            .route("/ingredient/create", post(create_ingredient_route))
            .with_state(state);

        Ok(App { router })
    }

    pub async fn serve(self, listener: tokio::net::TcpListener) -> color_eyre::Result<()> {
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
