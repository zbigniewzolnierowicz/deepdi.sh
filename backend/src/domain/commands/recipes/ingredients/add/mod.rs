use uuid::Uuid;

use crate::domain::{
    entities::recipe::{IngredientAmountData, IngredientWithAmount, Recipe},
    repositories::{
        ingredients::{errors::GetIngredientByIdError, IngredientRepositoryService},
        recipe::{
            errors::{
                AddIngredientIntoRecipeError as AddIngredientIntoRecipeErrorInternal,
                GetRecipeByIdError,
            },
            RecipeRepositoryService,
        },
    },
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum AddIngredientToRecipeError {
    #[error(transparent)]
    GetIngredient(#[from] GetIngredientByIdError),

    #[error(transparent)]
    GetRecipe(#[from] GetRecipeByIdError),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl From<AddIngredientIntoRecipeErrorInternal> for AddIngredientToRecipeError {
    fn from(value: AddIngredientIntoRecipeErrorInternal) -> Self {
        match value {
            e => e.into(),
        }
    }
}

pub async fn add_ingredient_to_recipe(
    recipe_repo: RecipeRepositoryService,
    ingredient_repo: IngredientRepositoryService,
    recipe_id: &Uuid,
    ingredient_amount: IngredientAmountData,
) -> Result<Recipe, AddIngredientToRecipeError> {
    let recipe = recipe_repo.get_by_id(recipe_id).await?;
    let ingredient = ingredient_repo
        .get_by_id(&ingredient_amount.ingredient_id)
        .await?;
    let IngredientAmountData {
        amount,
        notes,
        optional,
        ..
    } = ingredient_amount;

    recipe_repo
        .add_ingredient(
            &recipe,
            IngredientWithAmount {
                ingredient,
                amount,
                optional,
                notes,
            },
        )
        .await?;

    let recipe = recipe_repo.get_by_id(recipe_id).await?;

    Ok(recipe)
}

#[cfg(test)]
mod tests;
