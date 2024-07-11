use uuid::Uuid;

use crate::domain::{
    entities::ingredient::Ingredient,
    repositories::ingredients::{errors::GetIngredientByIdError, IngredientRepositoryService},
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum GetIngredientError {
    #[error("Ingredient with ID {0} was not found")]
    NotFound(Uuid),
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<GetIngredientByIdError> for GetIngredientError {
    fn from(value: GetIngredientByIdError) -> Self {
        match value {
            GetIngredientByIdError::NotFound(id) => Self::NotFound(id),
            e => Self::Internal(e.into()),
        }
    }
}

#[tracing::instrument("[QUERY] Get ingredient by ID", skip(repo))]
pub async fn get_ingredient_by_id(
    repo: IngredientRepositoryService,
    input: &Uuid,
) -> Result<Ingredient, GetIngredientError> {
    let result = repo.get_by_id(input).await?;

    Ok(result)
}

#[cfg(test)]
mod tests;
