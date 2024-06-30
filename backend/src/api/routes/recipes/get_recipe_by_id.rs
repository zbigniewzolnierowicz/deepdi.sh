use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::RecipeDTO;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::api::{errors::MakeError, AppState};
use crate::domain::queries::recipes::get_by_id::{get_recipe_by_id, GetRecipeError};

impl MakeError<String> for GetRecipeError {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for GetRecipeError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_json()).into_response()
    }
}

#[tracing::instrument("[ROUTE] Getting a recipe by ID", skip(recipe_repository))]
pub async fn get_recipe_by_id_route(
    State(AppState {
        recipe_repository, ..
    }): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> Result<Json<RecipeDTO>, GetRecipeError> {
    let result = get_recipe_by_id(recipe_repository, &recipe_id).await?;

    Ok(axum::Json(result.into()))
}
