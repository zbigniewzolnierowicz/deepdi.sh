use uuid::Uuid;

use crate::domain::repositories::recipe::errors::DeleteRecipeError as DeleteRecipeErrorInternal;
use crate::domain::repositories::recipe::RecipeRepositoryService;

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum DeleteRecipeError {
    #[error("Could not found the recipe with the ID {0}")]
    NotFound(Uuid),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

impl From<DeleteRecipeErrorInternal> for DeleteRecipeError {
    fn from(value: DeleteRecipeErrorInternal) -> Self {
        match value {
            DeleteRecipeErrorInternal::NotFound(id) => Self::NotFound(id),
            DeleteRecipeErrorInternal::UnknownError(err) => Self::Unknown(err),
        }
    }
}

pub async fn delete_recipe(
    recipe_repo: RecipeRepositoryService,
    input: &Uuid,
) -> Result<(), DeleteRecipeError> {
    recipe_repo
        .delete(input)
        .await
        .map_err(DeleteRecipeError::from)?;

    Ok(())
}

#[cfg(test)]
mod tests;
