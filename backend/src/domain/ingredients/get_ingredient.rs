use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    entities::ingredient::Ingredient, repositories::ingredients::{IngredientRepository, IngredientRepositoryError},
};

#[derive(thiserror::Error, Debug)]
pub enum GetIngredientError {
    #[error("Ingredient with ID {0} was not found")]
    NotFound(Uuid),
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<IngredientRepositoryError> for GetIngredientError {
    fn from(value: IngredientRepositoryError) -> Self {
        match value {
            IngredientRepositoryError::NotFound(id) => Self::NotFound(id),
            e => Self::Internal(e.into())
        }
    }
}

pub async fn get_ingredient_by_id(
    repo: Arc<dyn IngredientRepository>,
    input: Uuid,
) -> Result<Ingredient, GetIngredientError> {
    let result = repo
        .get_by_id(input)
        .await?;

    Ok(result)
}
