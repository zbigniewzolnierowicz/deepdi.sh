use std::sync::Arc;

use crate::domain::{
    entities::ingredient::Ingredient,
    repositories::ingredients::{base::IngredientRepository, errors::IngredientRepositoryError},
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum GetAllIngredientsError {
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<IngredientRepositoryError> for GetAllIngredientsError {
    fn from(value: IngredientRepositoryError) -> Self {
        Self::Internal(value.into())
    }
}

pub async fn get_all_ingredients(
    repo: Arc<dyn IngredientRepository>,
) -> Result<Vec<Ingredient>, GetAllIngredientsError> {
    Ok(repo.get_all().await?)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::domain::{
        entities::ingredient::types::{DietFriendly, IngredientDescription, IngredientName, WhichDiets},
        repositories::ingredients::InMemoryIngredientRepository,
    };

    use super::*;

    #[tokio::test]
    async fn returns_empty_vec_when_no_items_inside() {
        // GIVEN
        let repo = Arc::new(InMemoryIngredientRepository::new());

        // WHEN
        let result = get_all_ingredients(repo).await.unwrap();

        // THEN
        assert_eq!(result, vec![]);
    }

    #[tokio::test]
    async fn returns_vec_of_items_inside() {
        // GIVEN
        let repo = Arc::new(InMemoryIngredientRepository::new());
        let given_1 = Ingredient {
            id: Uuid::now_v7(),
            name: IngredientName("Tomato".into()),
            description: IngredientDescription("Description of a tomato".into()),
            diet_friendly: vec![DietFriendly::Vegan, DietFriendly::Vegetarian].into(),
        };

        let given_2 = Ingredient {
            id: Uuid::now_v7(),
            name: IngredientName("Meat fries".into()),
            description: IngredientDescription(
                "Description of meat fries (whatever they are)".into(),
            ),
            diet_friendly: WhichDiets(vec![]),
        };

        repo.insert(given_1.clone()).await.unwrap();
        repo.insert(given_2.clone()).await.unwrap();

        // WHEN
        let result = get_all_ingredients(repo).await.unwrap();

        // THEN
        assert_eq!(result, vec![given_1, given_2]);
    }
}
