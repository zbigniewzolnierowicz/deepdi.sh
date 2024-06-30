use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::{IngredientAmountDTO, RecipeDTO};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::{errors::MakeError, AppState},
    domain::{
        commands::recipes::ingredients::add::{
            add_ingredient_to_recipe, AddIngredientToRecipeError,
        },
        repositories::{
            ingredients::errors::GetIngredientByIdError, recipe::errors::GetRecipeByIdError,
        },
    },
};

impl MakeError<String> for AddIngredientToRecipeError {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::GetIngredient(GetIngredientByIdError::NotFound(_)) => StatusCode::BAD_REQUEST,
            Self::GetRecipe(GetRecipeByIdError::NotFound(_)) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for AddIngredientToRecipeError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_json()).into_response()
    }
}

pub async fn add_ingredient_to_recipe_route(
    State(AppState {
        recipe_repository,
        ingredient_repository,
        ..
    }): State<AppState>,
    Path(recipe_id): Path<Uuid>,
    Json(body): Json<IngredientAmountDTO>,
) -> Result<Json<RecipeDTO>, AddIngredientToRecipeError> {
    let ingredient_to_add = body.into();
    let result = add_ingredient_to_recipe(
        recipe_repository,
        ingredient_repository,
        &recipe_id,
        ingredient_to_add,
    )
    .await?;

    Ok(axum::Json(result.into()))
}
