use uuid::Uuid;

use crate::domain::{
    entities::recipe::{IngredientAmountData, Recipe},
    repositories::{
        ingredients::errors::GetIngredientByIdError,
        recipe::{errors::GetRecipeByIdError, RecipeRepositoryService},
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

pub async fn add_ingredient_to_recipe(
    recipe_repo: RecipeRepositoryService,
    recipe_id: &Uuid,
    ingredient_amount: IngredientAmountData,
) -> Result<Recipe, AddIngredientToRecipeError> {
    todo!()
}
