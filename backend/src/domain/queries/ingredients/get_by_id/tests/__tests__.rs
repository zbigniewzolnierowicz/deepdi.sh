use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    entities::ingredient::{types::WhichDiets, Ingredient},
    queries::ingredients::get_by_id::{get_ingredient_by_id, GetIngredientError},
    repositories::ingredients::{IngredientRepository, IngredientRepositoryService},
};

pub async fn get_by_id_returns_ingredient(repo: impl IngredientRepository) {
    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    })
    .await
    .unwrap();

    let result = repo.get_by_id(&Uuid::from_u128(1)).await.unwrap();
    assert_eq!(result.name, "Ingredient name".try_into().unwrap());
    assert_eq!(
        result.description,
        "Ingredient description".try_into().unwrap()
    );
}

pub async fn get_by_id_returns_error_when_missing(repo: impl IngredientRepository) {
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let result = get_ingredient_by_id(repo, &Uuid::from_u128(1))
        .await
        .unwrap_err();

    assert!(matches!(result, GetIngredientError::NotFound(id) if id == Uuid::from_u128(1)));
}
