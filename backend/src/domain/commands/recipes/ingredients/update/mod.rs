use thiserror::Error;
use uuid::Uuid;

use crate::domain::{
    entities::recipe::{IngredientUnit, Recipe},
    repositories::recipe::{
        errors::{
            GetRecipeByIdError,
            UpdateIngredientInRecipeError as UpdateIngredientInRecipeErrorInternal,
        },
        RecipeRepositoryService,
    },
};

#[derive(Error, Debug, strum::AsRefStr)]
pub enum UpdateIngredientInRecipeError {
    #[error(transparent)]
    GetRecipe(#[from] GetRecipeByIdError),

    #[error("Could not find ingredient with ID {0} in this recipe.")]
    MissingIngredient(Uuid),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl From<UpdateIngredientInRecipeErrorInternal> for UpdateIngredientInRecipeError {
    fn from(value: UpdateIngredientInRecipeErrorInternal) -> Self {
        Self::Unknown(value.into())
    }
}

pub async fn update_ingredient_in_recipe(
    recipe_repo: RecipeRepositoryService,
    recipe_id: &Uuid,
    ingredient_id: &Uuid,
    amount: IngredientUnit,
) -> Result<Recipe, UpdateIngredientInRecipeError> {
    let recipe = recipe_repo.get_by_id(recipe_id).await?;

    let ingredient_in_recipe = &recipe
        .ingredients
        .iter()
        .find(|x| x.ingredient.id == *ingredient_id)
        .ok_or(UpdateIngredientInRecipeError::MissingIngredient(
            *ingredient_id,
        ))?;

    recipe_repo
        .update_ingredient_amount(&recipe, ingredient_in_recipe, &amount)
        .await?;

    let recipe = recipe_repo.get_by_id(recipe_id).await?;

    Ok(recipe)
}

#[cfg(test)]
mod tests;
