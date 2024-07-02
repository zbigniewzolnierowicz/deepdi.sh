use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::{errors::MakeError, AppState},
    domain::commands::recipes::ingredients::delete::{
        delete_ingredient_from_recipe, DeleteIngredientFromRecipeError,
    },
};

impl MakeError<String> for DeleteIngredientFromRecipeError {
    fn get_status_code(&self) -> StatusCode {
        match self {
            DeleteIngredientFromRecipeError::RecipeNotFoundError(_) => StatusCode::NOT_FOUND,
            DeleteIngredientFromRecipeError::RecipeHasNoIngredientError(_) => {
                StatusCode::BAD_REQUEST
            }
            DeleteIngredientFromRecipeError::LastIngredientError => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for DeleteIngredientFromRecipeError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_json()).into_response()
    }
}

pub async fn delete_ingredient_from_recipe_route(
    State(AppState {
        recipe_repository, ..
    }): State<AppState>,
    Path((recipe_id, ingredient_id)): Path<(Uuid, Uuid)>,
) -> Result<(), DeleteIngredientFromRecipeError> {
    delete_ingredient_from_recipe(recipe_repository, &recipe_id, &ingredient_id).await?;

    Ok(())
}
