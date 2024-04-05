mod routes;

use std::sync::Arc;

use crate::domain::repositories::ingredients::{
    base::{IngredientRepositoryService, IngredientRepository}, in_memory::InMemoryIngredientRepository,
    postgres::PostgresIngredientRepository,
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
    pub ingredient_repository: IngredientRepositoryService,
}

impl App {
    fn get_router() -> Router<AppState> {
        Router::new()
            .route("/ingredient/create", post(create_ingredient_route))
            .route("/ingredient/:id", get(get_ingredient_by_id_route))
            .route("/ingredient", get(get_all_ingredients_route))
    }

    pub async fn with_in_memory() -> color_eyre::Result<Self> {
        Self::new(InMemoryIngredientRepository::new()).await
    }

    pub async fn with_db(db: PgPool) -> color_eyre::Result<Self> {
        Self::new(PostgresIngredientRepository::new(db)).await
    }

    async fn new<I: IngredientRepository + 'static>(irs: I) -> color_eyre::Result<Self> {
        let ingredient_repository: IngredientRepositoryService = Arc::new(Box::new(irs));
        let state = AppState {
            ingredient_repository,
        };
        let router = Self::get_router().with_state(state);

        Ok(App { router })
    }

    pub async fn serve(self, listener: tokio::net::TcpListener) -> color_eyre::Result<()> {
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
