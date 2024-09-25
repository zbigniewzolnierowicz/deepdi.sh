use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::{errors::MakeError, AppState},
    domain::commands::recipes::delete::{delete_recipe, DeleteRecipeError},
};

impl MakeError<String> for DeleteRecipeError {
    fn get_kind(&self) -> String {
        self.as_ref().to_string()
    }
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

impl IntoResponse for DeleteRecipeError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_json()).into_response()
    }
}

#[tracing::instrument("[ROUTE] Deleting a recipe", skip(recipe_repository))]
pub async fn delete_recipe_route(
    State(AppState {
        recipe_repository, ..
    }): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> Result<(), DeleteRecipeError> {
    delete_recipe(recipe_repository, &recipe_id).await?;

    Ok(())
}
