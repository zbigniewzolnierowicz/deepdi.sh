use axum::{extract::State, Json};
use common::IngredientDTO;

use crate::{
    api::AppState,
    domain::queries::ingredients::get_all::{get_all_ingredients, GetAllIngredientsError},
};

#[tracing::instrument(
    "[ROUTE] Getting all available ingredients",
    skip(ingredient_repository)
)]
pub async fn get_all_ingredients_route(
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
) -> Result<Json<Vec<IngredientDTO>>, GetAllIngredientsError> {
    let result = get_all_ingredients(ingredient_repository).await?;

    Ok(Json(result.iter().map(IngredientDTO::from).collect()))
}
