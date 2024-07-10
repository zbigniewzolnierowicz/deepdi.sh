use strum::AsRefStr;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::recipe::errors::ValidationError;
use crate::domain::repositories::recipe::errors::{
    DeleteIngredientFromRecipeError as DeleteIngredientFromRecipeErrorInternal, GetRecipeByIdError,
};
use crate::domain::repositories::recipe::RecipeRepositoryService;

#[derive(Error, Debug, AsRefStr)]
pub enum DeleteIngredientFromRecipeError {
    #[error("Could not found recipe with ID {0}")]
    RecipeNotFoundError(Uuid),

    #[error("The recipe has no ingredient with ID of {0}")]
    RecipeHasNoIngredientError(Uuid),

    #[error("There is only one ingredient in the recipe. A recipe should have one ingredient at minimum.")]
    LastIngredientError,

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl From<DeleteIngredientFromRecipeErrorInternal> for DeleteIngredientFromRecipeError {
    fn from(value: DeleteIngredientFromRecipeErrorInternal) -> Self {
        match value {
            DeleteIngredientFromRecipeErrorInternal::ValidationError(
                ValidationError::EmptyField(field),
            ) if field == vec!["steps"] => Self::LastIngredientError,
            e => e.into(),
        }
    }
}

impl From<GetRecipeByIdError> for DeleteIngredientFromRecipeError {
    fn from(value: GetRecipeByIdError) -> Self {
        match value {
            GetRecipeByIdError::NotFound(id) => Self::RecipeNotFoundError(id),
            e => e.into(),
        }
    }
}

pub async fn delete_ingredient_from_recipe(
    recipe_repo: RecipeRepositoryService,
    recipe_id: &Uuid,
    ingredient_id: &Uuid,
) -> Result<(), DeleteIngredientFromRecipeError> {
    let recipe = recipe_repo.get_by_id(recipe_id).await?;

    if recipe.ingredients.len() == 1 {
        return Err(DeleteIngredientFromRecipeError::LastIngredientError);
    };

    let ingredient_in_recipe = &recipe
        .ingredients
        .iter()
        .find(|x| x.ingredient.id == *ingredient_id)
        .ok_or_else(|| {
            DeleteIngredientFromRecipeError::RecipeHasNoIngredientError(*ingredient_id)
        })?;

    recipe_repo
        .delete_ingredient(&recipe, &ingredient_in_recipe)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests;
