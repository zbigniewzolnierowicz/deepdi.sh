use super::*;
use crate::domain::entities::ingredient::types::WhichDiets;

pub async fn deleting_works(repo: impl IngredientRepository) {
    let input = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    };

    let insert_result = repo.insert(input).await.unwrap();
    repo.delete(insert_result.id).await.unwrap();
}

pub async fn deleting_nonexistent_ingredient_errors(repo: impl IngredientRepository) {
    let error = repo.delete(Uuid::nil()).await.unwrap_err();

    assert!(matches!(
        error,
        DeleteIngredientError::Get(GetIngredientByIdError::NotFound(id)) if id == Uuid::nil()
    ));
}
