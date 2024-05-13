use axum::{extract::State, response::IntoResponse, Json};
use common::{
    CreateRecipeDTO, IngredientUnitDTO, IngredientWithAmountDTO, RecipeDTO, ServingsTypeDTO,
};
use reqwest::StatusCode;

use crate::{
    api::AppState,
    domain::{
        commands::recipes::create::{
            create_recipe, CreateRecipe, CreateRecipeError, IngredientAmountData,
        },
        entities::recipe::{IngredientUnit, IngredientWithAmount, Recipe, ServingsType},
    },
};
impl IntoResponse for CreateRecipeError {
    fn into_response(self) -> axum::response::Response {
        let error_type: &str = self.as_ref();
        (
            StatusCode::BAD_REQUEST,
            axum::Json(common::error::ErrorMessage::new(
                error_type,
                self.to_string(),
            )),
        )
            .into_response()
    }
}

impl From<ServingsTypeDTO> for ServingsType {
    fn from(value: ServingsTypeDTO) -> Self {
        match value {
            ServingsTypeDTO::Exact(a) => Self::Exact(a),
            ServingsTypeDTO::FromTo(a, b) => Self::FromTo(a, b),
        }
    }
}

impl From<IngredientUnitDTO> for IngredientUnit {
    fn from(value: IngredientUnitDTO) -> Self {
        match value {
            IngredientUnitDTO::Cups(amount) => Self::Cups(amount),
            IngredientUnitDTO::Grams(amount) => Self::Grams(amount),
            IngredientUnitDTO::Mililiters(amount) => Self::Mililiters(amount),
            IngredientUnitDTO::Teaspoons(amount) => Self::Teaspoons(amount),
            IngredientUnitDTO::Other { amount, unit } => Self::Other { amount, unit },
        }
    }
}

impl From<IngredientUnit> for IngredientUnitDTO {
    fn from(value: IngredientUnit) -> Self {
        match value {
            IngredientUnit::Cups(amount) => Self::Cups(amount),
            IngredientUnit::Grams(amount) => Self::Grams(amount),
            IngredientUnit::Mililiters(amount) => Self::Mililiters(amount),
            IngredientUnit::Teaspoons(amount) => Self::Teaspoons(amount),
            IngredientUnit::Other { amount, unit } => Self::Other { amount, unit },
        }
    }
}

impl From<IngredientWithAmount> for IngredientWithAmountDTO {
    fn from(value: IngredientWithAmount) -> Self {
        Self {
            ingredient: value.ingredient.into(),
            optional: value.optional,
            notes: value.notes,
            amount: value.amount.into(),
        }
    }
}

impl From<ServingsType> for ServingsTypeDTO {
    fn from(value: ServingsType) -> Self {
        match value {
            ServingsType::Exact(a) => Self::Exact(a),
            ServingsType::FromTo(a, b) => Self::FromTo(a, b),
        }
    }
}

impl From<Recipe> for RecipeDTO {
    fn from(value: Recipe) -> Self {
        Self {
            ingredients: value.ingredients.into_iter().map(|i| i.into()).collect(),
            name: value.name,
            description: value.description,
            steps: value.steps,
            time: value
                .time
                .into_iter()
                .map(|(k, v)| (k, v.as_secs()))
                .collect(),
            servings: value.servings.into(),
        }
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
) -> Result<Json<RecipeDTO>, CreateRecipeError> {
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
        name: &body.name,
        description: &body.description,
        servings: body.servings.into(),
        time: body
            .time
            .into_iter()
            .map(|(k, v)| (k, std::time::Duration::from_secs(v)))
            .collect(),
        steps: body.steps,
        ingredients,
    };
    let result = create_recipe(recipe_repository, ingredient_repository, &input).await?;

    Ok(Json(result.into()))
}
