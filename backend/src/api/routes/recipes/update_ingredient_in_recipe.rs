use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::{IngredientUnitDTO, RecipeDTO};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::{errors::MakeError, AppState},
    domain::{
        commands::recipes::ingredients::update::{
            update_ingredient_in_recipe, UpdateIngredientInRecipeError,
        },
        entities::recipe::IngredientUnit,
        repositories::recipe::errors::GetRecipeByIdError,
    },
};

impl MakeError<String> for UpdateIngredientInRecipeError {
    fn get_status_code(&self) -> StatusCode {
        match self {
            UpdateIngredientInRecipeError::MissingIngredient(_)
            | UpdateIngredientInRecipeError::GetRecipe(GetRecipeByIdError::NotFound(_)) => {
                StatusCode::NOT_FOUND
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for UpdateIngredientInRecipeError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_json()).into_response()
    }
}

pub async fn update_ingredient_in_recipe_route(
    State(AppState {
        recipe_repository, ..
    }): State<AppState>,
    Path((recipe_id, ingredient_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<IngredientUnitDTO>,
) -> Result<Json<RecipeDTO>, UpdateIngredientInRecipeError> {
    let amount: IngredientUnit = body.into();
    let recipe =
        update_ingredient_in_recipe(recipe_repository, &recipe_id, &ingredient_id, amount).await?;

    Ok(Json(recipe.into()))
}
