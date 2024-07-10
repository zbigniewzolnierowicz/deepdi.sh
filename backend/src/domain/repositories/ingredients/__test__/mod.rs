use super::*;
use crate::domain::entities::ingredient::types::{
    IngredientDescription, IngredientName, WhichDiets,
};

use pretty_assertions::assert_eq;

pub async fn insert_ingredient_that_already_exists_fails_id(repo: impl IngredientRepository) {
    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    })
    .await
    .unwrap();

    let result = repo
        .insert(Ingredient {
            id: Uuid::from_u128(1),
            name: "Ingredient name 2".try_into().unwrap(),
            description: "Ingredient description 2".try_into().unwrap(),
            diet_friendly: WhichDiets::new(),
        })
        .await
        .unwrap_err();

    assert!(matches!(
        result,
        InsertIngredientError::Conflict(fieldname) if fieldname == "id"
    ))
}

pub async fn insert_ingredient_that_already_exists_fails_name(repo: impl IngredientRepository) {
    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    })
    .await
    .unwrap();

    let result = repo
        .insert(Ingredient {
            id: Uuid::from_u128(2),
            name: "Ingredient name".try_into().unwrap(),
            description: "Ingredient description".try_into().unwrap(),
            diet_friendly: WhichDiets::new(),
        })
        .await
        .unwrap_err();

    assert!(matches!(
        result,
        InsertIngredientError::Conflict(fieldname) if fieldname == "name"
    ))
}

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
    let result = repo.get_by_id(&Uuid::from_u128(1)).await.unwrap_err();

    assert!(matches!(result, GetIngredientByIdError::NotFound(id) if id == Uuid::from_u128(1)));
}

pub async fn get_all_returns_all_ingredients(repo: impl IngredientRepository) {
    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    })
    .await
    .unwrap();

    repo.insert(Ingredient {
        id: Uuid::from_u128(2),
        name: "Ingredient name 2".try_into().unwrap(),
        description: "Ingredient description 2".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    })
    .await
    .unwrap();

    let mut result = repo.get_all().await.unwrap();

    assert_eq!(result.len(), 2);

    result.sort_by_key(|x| x.id);

    for (index, entry) in result.iter().enumerate() {
        let index = index + 1;

        assert_eq!(entry.id, Uuid::from_u128(index.try_into().unwrap()));
        assert_eq!(
            entry.name,
            IngredientName(format!("Ingredient name {index}"))
        );
        assert_eq!(
            entry.description,
            IngredientDescription(format!("Ingredient description {index}"))
        );
    }
}

pub async fn get_all_returns_empty_vec(repo: impl IngredientRepository) {
    let result = repo.get_all().await.unwrap();

    assert_eq!(result, vec![]);
}

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
