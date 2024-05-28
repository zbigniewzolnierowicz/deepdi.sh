use axum::{extract::State, Json};
use common::{CreateIngredientDTO, IngredientDTO};

use crate::{
    api::AppState,
    domain::commands::ingredients::create::{
        create_ingredient, CreateIngredient, CreateIngredientError,
    },
};

#[tracing::instrument("[ROUTE] Creating a new ingredient", skip(ingredient_repository))]
pub async fn create_ingredient_route(
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
    Json(body): Json<CreateIngredientDTO>,
) -> Result<Json<IngredientDTO>, CreateIngredientError> {
    let input = CreateIngredient {
        name: &body.name,
        description: &body.description,
        diet_friendly: body.diet_friendly.unwrap_or_default(),
    };
    let result = create_ingredient(ingredient_repository, &input).await?;

    Ok(Json(result.into()))
}
