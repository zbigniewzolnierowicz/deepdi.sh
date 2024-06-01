use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::{IngredientDTO, UpdateIngredientDTO};
use uuid::Uuid;

use crate::{
    api::{errors::MakeError, AppState},
    domain::commands::ingredients::update::{update_ingredient, UpdateIngredientError},
};

impl MakeError<String> for UpdateIngredientError {
    fn get_message(&self) -> String {
        self.to_string()
    }
    fn get_status_code(&self) -> reqwest::StatusCode {
        match self {
            Self::NotFound(_) => reqwest::StatusCode::NOT_FOUND,
            _ => reqwest::StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl IntoResponse for UpdateIngredientError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_message()).into_response()
    }
}

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
