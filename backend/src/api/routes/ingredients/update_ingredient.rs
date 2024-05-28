use axum::{
    extract::{Path, State},
    Json,
};
use common::{IngredientDTO, UpdateIngredientDTO};
use uuid::Uuid;

use crate::{
    api::AppState,
    domain::commands::ingredients::update::{update_ingredient, UpdateIngredientError},
};

#[tracing::instrument("[ROUTE] Updating an existing ingredient", skip(ingredient_repository))]
pub async fn update_ingredient_route(
    Path(ingredient_id): Path<Uuid>,
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
    Json(body): Json<UpdateIngredientDTO>,
) -> Result<Json<IngredientDTO>, UpdateIngredientError> {
    let result = update_ingredient(ingredient_repository, ingredient_id, &body.into()).await?;

    Ok(Json(result.into()))
}
