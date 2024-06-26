use super::*;
use crate::domain::entities::ingredient::{
    errors::ValidationError,
    types::{IngredientDescription, IngredientName, WhichDiets},
};

use pretty_assertions::assert_eq;

#[tokio::test]
async fn insert_ingredient_succeeds() {
    let repo = InMemoryIngredientRepository::new();

    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    })
    .await
    .unwrap();

    let lock = repo.0.lock().unwrap();
    let ingredient: Vec<_> = lock.values().collect();
    assert_eq!(ingredient.len(), 1);
}

#[tokio::test]
async fn insert_ingredient_that_already_exists_fails_id() {
    let repo = InMemoryIngredientRepository::new();

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

#[tokio::test]
async fn insert_ingredient_that_already_exists_fails_name() {
    let repo = InMemoryIngredientRepository::new();

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

#[tokio::test]
async fn get_by_id_returns_ingredient() {
    let repo = InMemoryIngredientRepository::new();
    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    })
    .await
    .unwrap();

    let result = repo.get_by_id(Uuid::from_u128(1)).await.unwrap();
    assert_eq!(result.name, "Ingredient name".try_into().unwrap());
    assert_eq!(
        result.description,
        "Ingredient description".try_into().unwrap()
    );
}

#[tokio::test]
async fn get_by_id_returns_error_when_missing() {
    let repo = InMemoryIngredientRepository::new();

    let result = repo.get_by_id(Uuid::from_u128(1)).await.unwrap_err();

    assert!(matches!(result, GetIngredientByIdError::NotFound(id) if id == Uuid::from_u128(1)));
}

#[tokio::test]
async fn get_all_returns_all_ingredients() {
    let repo = InMemoryIngredientRepository::new();
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

#[tokio::test]
async fn get_all_returns_empty_vec() {
    let repo = InMemoryIngredientRepository::new();
    let result = repo.get_all().await.unwrap();

    assert_eq!(result, vec![]);
}

#[tokio::test]
async fn updating_an_ingredient_success() {
    let repo = InMemoryIngredientRepository::new();

    let input = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    };
    repo.insert(input.clone()).await.unwrap();

    let result = repo
        .update(
            input.id,
            IngredientChangeset {
                name: Some(IngredientName("Ingredient name changed".to_string())),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    assert_eq!(
        result,
        Ingredient {
            name: IngredientName("Ingredient name changed".to_string()),
            ..input
        }
    )
}

#[tokio::test]
async fn updating_with_empty_changeset_fails() {
    let repo = InMemoryIngredientRepository::new();

    let input = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    };
    repo.insert(input.clone()).await.unwrap();

    let error = repo
        .update(input.id, IngredientChangeset::default())
        .await
        .unwrap_err();

    assert!(
        matches!(error, UpdateIngredientError::ValidationError(ValidationError::EmptyField(fields)) if fields == vec!["name", "description", "diet_friendly"])
    );
}

#[tokio::test]
async fn updating_a_missing_file_fails() {
    let repo = InMemoryIngredientRepository::new();

    let error = repo
        .update(
            Uuid::from_u128(1),
            IngredientChangeset {
                name: Some(IngredientName(
                    "This will fail, so this doesn't matter".to_string(),
                )),
                ..Default::default()
            },
        )
        .await
        .unwrap_err();

    assert!(
        matches!(error, UpdateIngredientError::Get(GetIngredientByIdError::NotFound(id)) if id == Uuid::from_u128(1))
    );
}

#[tokio::test]
async fn deleting_works() {
    let repo = InMemoryIngredientRepository::new();

    let input = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    };

    let insert_result = repo.insert(input).await.unwrap();
    repo.delete(insert_result.id).await.unwrap();
}

#[tokio::test]
async fn deleting_nonexistent_ingredient_errors() {
    let repo = InMemoryIngredientRepository::new();
    let error = repo.delete(Uuid::nil()).await.unwrap_err();

    assert!(matches!(
        error,
        DeleteIngredientError::Get(GetIngredientByIdError::NotFound(id)) if id == Uuid::nil()
    ));
}
