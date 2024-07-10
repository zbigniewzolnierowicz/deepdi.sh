use uuid::Uuid;

use crate::domain::repositories::recipe::errors::{
    DeleteRecipeError as DeleteRecipeErrorInternal, GetRecipeByIdError,
};
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
        Self::Unknown(value.into())
    }
}

impl From<GetRecipeByIdError> for DeleteRecipeError {
    fn from(value: GetRecipeByIdError) -> Self {
        match value {
            GetRecipeByIdError::NotFound(id) => Self::NotFound(id),
            e => e.into(),
        }
    }
}

pub async fn delete_recipe(
    recipe_repo: RecipeRepositoryService,
    input: &Uuid,
) -> Result<(), DeleteRecipeError> {
    let recipe = recipe_repo.get_by_id(input).await?;

    recipe_repo.delete(&recipe).await?;

    Ok(())
}

#[cfg(test)]
mod tests;
