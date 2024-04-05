mod routes;

use std::sync::Arc;

use crate::{
    configuration::Settings,
    domain::repositories::ingredients::{
        base::IngredientRepository, postgres::PostgresIngredientRepository,
    },
};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use self::routes::{
    all_ingredients::get_all_ingredients_route, create_ingredient::create_ingredient_route,
    get_ingredient_by_id::get_ingredient_by_id_route,
};

pub struct App {
    router: Router,
}

#[derive(Clone)]
pub struct AppState {
    pub ingredient_repository: Arc<dyn IngredientRepository>,
}

impl App {
    pub async fn new(config: &Settings) -> color_eyre::Result<Self> {
        let db = PgPool::connect_lazy_with(config.database.with_db());
        let ingredient_repository = Arc::new(PostgresIngredientRepository::new(db));
        let state = AppState {
            ingredient_repository,
        };
        let router = Router::new()
            .route("/ingredient/create", post(create_ingredient_route))
            .route("/ingredient/:id", get(get_ingredient_by_id_route))
            .route("/ingredient", get(get_all_ingredients_route))
            .with_state(state);

        Ok(App { router })
    }

    pub async fn serve(self, listener: tokio::net::TcpListener) -> color_eyre::Result<()> {
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
