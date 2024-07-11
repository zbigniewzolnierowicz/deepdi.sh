use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::IngredientDTO;
use uuid::Uuid;

use crate::{
    api::{errors::MakeError, AppState},
    domain::queries::ingredients::get_by_id::{get_ingredient_by_id, GetIngredientError},
};

impl MakeError<String> for GetIngredientError {
    fn get_message(&self) -> String {
        self.to_string()
    }
    fn get_status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::BAD_REQUEST
    }
}

impl IntoResponse for GetIngredientError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_message()).into_response()
    }
}

#[tracing::instrument("[ROUTE] Getting ingredient by ID", skip(ingredient_repository))]
pub async fn get_ingredient_by_id_route(
    Path(ingredient_id): Path<Uuid>,
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
) -> Result<Json<IngredientDTO>, GetIngredientError> {
    // TODO: Switch to storing the amount of recipes with certain ingredient in some sort of cache
    let result = get_ingredient_by_id(ingredient_repository, &ingredient_id).await?;

    Ok(Json(result.into()))
}
