use crate::domain::{
    entities::ingredient::Ingredient,
    repositories::ingredients::{
        errors::GetAllIngredientsError as GetAllIngredientsErrorInternal,
        IngredientRepositoryService,
    },
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum GetAllIngredientsError {
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<GetAllIngredientsErrorInternal> for GetAllIngredientsError {
    fn from(value: GetAllIngredientsErrorInternal) -> Self {
        Self::Internal(value.into())
    }
}

#[tracing::instrument("[QUERY] Get all ingredients", skip(repo))]
pub async fn get_all_ingredients(
    repo: IngredientRepositoryService,
) -> Result<Vec<Ingredient>, GetAllIngredientsError> {
    repo.get_all().await.map_err(GetAllIngredientsError::from)
}

#[cfg(test)]
mod tests;

