use axum::{extract::State, Json};
use common::{CreateRecipeDTO, RecipeDTO};

use crate::{
    api::AppState,
    domain::commands::recipes::create::{
        create_recipe, CreateRecipe, CreateRecipeError, IngredientAmountData,
    },
};

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
