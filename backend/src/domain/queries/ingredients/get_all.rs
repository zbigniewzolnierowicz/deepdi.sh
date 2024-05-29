use axum::response::IntoResponse;
use reqwest::StatusCode;

use crate::domain::{
    entities::ingredient::Ingredient,
    repositories::ingredients::{errors::IngredientRepositoryError, IngredientRepositoryService},
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum GetAllIngredientsError {
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl IntoResponse for GetAllIngredientsError {
    fn into_response(self) -> axum::response::Response {
        let error_type: &str = self.as_ref();
        (
            StatusCode::BAD_REQUEST,
            axum::Json(common::error::ErrorMessage::new(
                error_type,
                self.to_string(),
            )),
        )
            .into_response()
    }
}

impl From<IngredientRepositoryError> for GetAllIngredientsError {
    fn from(value: IngredientRepositoryError) -> Self {
        Self::Internal(value.into())
    }
}

#[tracing::instrument("[QUERY] Get all ingredients", skip(repo))]
pub async fn get_all_ingredients(
    repo: IngredientRepositoryService,
) -> Result<Vec<Ingredient>, GetAllIngredientsError> {
    Ok(repo.get_all().await?)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::domain::{
        entities::ingredient::types::{
            DietFriendly, IngredientDescription, IngredientName, WhichDiets,
        },
        repositories::ingredients::InMemoryIngredientRepository,
    };

    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn returns_empty_vec_when_no_items_inside() {
        // GIVEN
        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));

        // WHEN
        let result = get_all_ingredients(repo).await.unwrap();

        // THEN
        assert_eq!(result, vec![]);
    }

    #[tokio::test]
    async fn returns_vec_of_items_inside() {
        // GIVEN
        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));
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
            diet_friendly: WhichDiets::new(),
        };

        repo.insert(given_1.clone()).await.unwrap();
        repo.insert(given_2.clone()).await.unwrap();

        // WHEN
        let mut result = get_all_ingredients(repo).await.unwrap();
        result.sort_by_key(|k| k.id);

        let mut expected = vec![given_1, given_2];
        expected.sort_by_key(|k| k.id);

        // THEN
        assert_eq!(result, expected);
    }
}
