use axum::{extract::State, response::IntoResponse};
use common::{CreateRecipeDTO, RecipeDTO};
use reqwest::StatusCode;

use crate::api::errors::MakeError;
use crate::api::extract::Json;
use crate::api::AppState;
use crate::domain::commands::recipes::create::{create_recipe, CreateRecipe, CreateRecipeError};
use crate::domain::entities::recipe::IngredientAmountData;

impl MakeError<String> for CreateRecipeError {
    fn get_kind(&self) -> String {
        self.as_ref().to_string()
    }
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::IngredientsNotFound(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
}

impl IntoResponse for CreateRecipeError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_json()).into_response()
    }
}

#[tracing::instrument(
    "[ROUTE] Creating a new recipe",
    skip(ingredient_repository, recipe_repository)
)]
pub async fn create_recipe_route(
    State(AppState {
        ingredient_repository,
        recipe_repository,
        ..
    }): State<AppState>,
    Json(body): Json<CreateRecipeDTO>,
) -> Result<impl IntoResponse, CreateRecipeError> {
    let ingredients: Vec<IngredientAmountData> = body
        .ingredients
        .into_iter()
        .map(|i| IngredientAmountData {
            notes: i.notes,
            amount: i.amount.into(),
            optional: i.optional,
            ingredient_id: i.ingredient_id,
        })
        .collect();

    let input = CreateRecipe {
        name: body.name,
        description: body.description,
        servings: body.servings.into(),
        time: body
            .time
            .into_iter()
            .map(|(k, v)| (k, std::time::Duration::from_secs(v)))
            .collect(),
        steps: body.steps,
        ingredients,
    };

    let result: RecipeDTO = create_recipe(recipe_repository, ingredient_repository, &input)
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(result)))
}
