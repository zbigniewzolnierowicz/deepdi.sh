use axum::extract::Path;
use axum::{extract::State, response::IntoResponse, Json};
use common::{RecipeDTO, UpdateRecipeDTO};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::api::errors::MakeError;
use crate::api::AppState;
use crate::domain::commands::recipes::update::{update_recipe, UpdateRecipeError};

impl MakeError<String> for UpdateRecipeError {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for UpdateRecipeError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_json()).into_response()
    }
}

#[tracing::instrument("[ROUTE] Creating a new recipe", skip(recipe_repository))]
pub async fn update_recipe_route(
    State(AppState {
        recipe_repository, ..
    }): State<AppState>,
    Path(recipe_id): Path<Uuid>,
    Json(body): Json<UpdateRecipeDTO>,
) -> Result<Json<RecipeDTO>, UpdateRecipeError> {
    let recipe = update_recipe(recipe_repository, &recipe_id, body.into()).await?;

    Ok(axum::Json(recipe.try_into()?))
}
