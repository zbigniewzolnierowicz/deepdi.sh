use super::*;
use crate::{domain::entities::ingredient::types::WhichDiets, test_utils::ingredient_fixture};

pub async fn deleting_works(repo: impl IngredientRepository) {
    let input = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    };

    let insert_result = repo.insert(input).await.unwrap();
    repo.delete(insert_result).await.unwrap();
}

pub async fn deleting_nonexistent_ingredient_errors(repo: impl IngredientRepository) {
    let error = repo.delete(ingredient_fixture()).await.unwrap_err();

    assert!(matches!(
        error,
        DeleteIngredientError::Get(GetIngredientByIdError::NotFound(id)) if id == Uuid::nil()
    ));
}
