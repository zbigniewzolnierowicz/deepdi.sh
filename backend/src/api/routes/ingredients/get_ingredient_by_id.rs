use axum::{
    extract::{Path, State},
    Json,
};
use common::IngredientDTO;
use uuid::Uuid;

use crate::{
    api::AppState,
    domain::queries::ingredients::get_by_id::{get_ingredient_by_id, GetIngredientError},
};

#[tracing::instrument("[ROUTE] Getting ingredient by ID", skip(ingredient_repository))]
pub async fn get_ingredient_by_id_route(
    Path(ingredient_id): Path<Uuid>,
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
) -> Result<Json<IngredientDTO>, GetIngredientError> {
    let result = get_ingredient_by_id(ingredient_repository, ingredient_id).await?;

    Ok(Json(result.into()))
}
