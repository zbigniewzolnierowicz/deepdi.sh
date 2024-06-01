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
    input: Uuid,
) -> Result<Ingredient, GetIngredientError> {
    let result = repo.get_by_id(input).await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::{
        entities::ingredient::types::{DietFriendly, IngredientDescription, IngredientName},
        repositories::ingredients::InMemoryIngredientRepository,
    };

    use super::*;

    #[tokio::test]
    async fn getting_ingredient_works() {
        // GIVEN
        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));

        let given = Ingredient {
            id: Uuid::now_v7(),
            name: IngredientName("Tomato".into()),
            description: IngredientDescription("Description of a tomato".into()),
            diet_friendly: vec![DietFriendly::Vegan, DietFriendly::Vegetarian].into(),
        };
        let insert_result = repo.insert(given.clone()).await.unwrap();

        // WHEN
        let res = get_ingredient_by_id(repo, insert_result.id).await.unwrap();
        assert_eq!(res, insert_result);
    }

    #[tokio::test]
    async fn returns_error_if_no_ingredient_with_id() {
        // GIVEN
        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));
        let id = Uuid::from_u128(0);

        // WHEN
        let error = get_ingredient_by_id(repo, id).await.unwrap_err();

        assert!(matches!(error, GetIngredientError::NotFound(missing_id) if missing_id == id));
    }
}
