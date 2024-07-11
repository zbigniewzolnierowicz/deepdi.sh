use thiserror::Error;
use uuid::Uuid;

use crate::domain::repositories::{
    ingredients::{
        errors::{DeleteIngredientError as DeleteIngredientErrorInternal, GetIngredientByIdError},
        IngredientRepositoryService,
    },
    recipe::RecipeRepositoryService,
};

#[derive(Error, Debug)]
pub enum DeleteIngredientError {
    #[error("The ingredient with ID of {0} was not found.")]
    NotFound(Uuid),

    #[error("There are recipes that use this ingredient. Delete them first, then you will be able to delete this ingredient.")]
    InUseByRecipe,

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl From<DeleteIngredientErrorInternal> for DeleteIngredientError {
    fn from(value: DeleteIngredientErrorInternal) -> Self {
        Self::UnknownError(value.into())
    }
}

impl From<GetIngredientByIdError> for DeleteIngredientError {
    fn from(value: GetIngredientByIdError) -> Self {
        match value {
            GetIngredientByIdError::NotFound(id) => Self::NotFound(id),
            e => Self::UnknownError(e.into()),
        }
    }
}

#[tracing::instrument("[COMMAND] Deleting a new ingredient", skip(repo, recipe_repo))]
pub async fn delete_ingredient(
    repo: IngredientRepositoryService,
    recipe_repo: RecipeRepositoryService,
    input: &Uuid,
) -> Result<(), DeleteIngredientError> {
    let ingredient = repo.get_by_id(input).await?;
    let recipes_with_ingredient_exist = recipe_repo
        .recipes_containing_ingredient_exist(ingredient.clone())
        .await?;

    if recipes_with_ingredient_exist {
        return Err(DeleteIngredientError::InUseByRecipe);
    };

    repo.delete(ingredient).await?;

    Ok(())
}

#[cfg(test)]
mod tests;
