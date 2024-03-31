use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::AppState,
    domain::ingredients::create_ingredient::{create_ingredient, CreateIngredient},
};

#[derive(Deserialize)]
pub struct Body {
    pub name: String,
    pub description: String,
    pub diet_friendly: Vec<String>,
}

#[derive(Serialize)]
pub struct Response {
    pub id: Uuid,
}

pub async fn create_ingredient_route(
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
    Json(body): Json<Body>,
) -> axum::response::Result<Json<Response>> {
    let input = CreateIngredient {
        name: &body.name,
        description: &body.description,
        diet_friendly: body.diet_friendly,
    };
    let result = create_ingredient(ingredient_repository, &input).await?;

    Ok(axum::Json(Response { id: result.id }))
}
