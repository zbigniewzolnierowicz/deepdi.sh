use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    entities::ingredient::Ingredient,
    repositories::ingredients::{base::IngredientRepository, errors::IngredientRepositoryError},
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
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
            e => Self::Internal(e.into()),
        }
    }
}

pub async fn get_ingredient_by_id(
    repo: Arc<dyn IngredientRepository>,
    input: Uuid,
) -> Result<Ingredient, GetIngredientError> {
    let result = repo.get_by_id(input).await?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        entities::ingredient::types::{DietFriendly, IngredientDescription, IngredientName},
        repositories::ingredients::InMemoryIngredientRepository,
    };

    use super::*;

    #[tokio::test]
    async fn getting_ingredient_works() {
        // GIVEN
        let repo = Arc::new(InMemoryIngredientRepository::new());
        let given = Ingredient {
            id: Uuid::now_v7(),
            name: IngredientName("Tomato".into()),
            description: IngredientDescription("Description of a tomato".into()),
            diet_friendly: vec![DietFriendly::Vegan, DietFriendly::Vegetarian],
        };
        let insert_result = repo.insert(given.clone()).await.unwrap();

        // WHEN
        let res = get_ingredient_by_id(repo, insert_result.id).await.unwrap();
        assert_eq!(res, insert_result);
    }

    #[tokio::test]
    async fn returns_error_if_no_ingredient_with_id() {
        // GIVEN
        let repo = Arc::new(InMemoryIngredientRepository::new());
        let id = Uuid::from_u128(0);

        // WHEN
        match get_ingredient_by_id(repo, id).await {
            Err(GetIngredientError::NotFound(missing_id)) => assert_eq!(missing_id, id),
            _ => unreachable!(),
        }
    }
}
