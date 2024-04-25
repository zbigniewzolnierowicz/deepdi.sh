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
        diet_friendly: WhichDiets(vec![]),
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
        diet_friendly: WhichDiets(vec![]),
    })
    .await
    .unwrap();

    let result = repo
        .insert(Ingredient {
            id: Uuid::from_u128(1),
            name: "Ingredient name 2".try_into().unwrap(),
            description: "Ingredient description 2".try_into().unwrap(),
            diet_friendly: WhichDiets(vec![]),
        })
        .await
        .unwrap_err();

    match result {
        IngredientRepositoryError::Conflict(fieldname) => {
            assert_eq!(fieldname, "id");
        }
        _ => unreachable!(),
    };
}

#[tokio::test]
async fn insert_ingredient_that_already_exists_fails_name() {
    let repo = InMemoryIngredientRepository::new();

    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets(vec![]),
    })
    .await
    .unwrap();

    let result = repo
        .insert(Ingredient {
            id: Uuid::from_u128(2),
            name: "Ingredient name".try_into().unwrap(),
            description: "Ingredient description".try_into().unwrap(),
            diet_friendly: WhichDiets(vec![]),
        })
        .await
        .unwrap_err();

    match result {
        IngredientRepositoryError::Conflict(fieldname) => {
            assert_eq!(fieldname, "name");
        }
        _ => unreachable!(),
    };
}

#[tokio::test]
async fn get_by_id_returns_ingredient() {
    let repo = InMemoryIngredientRepository::new();
    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets(vec![]),
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

    match result {
        IngredientRepositoryError::NotFound(id) => {
            assert_eq!(id, Uuid::from_u128(1));
        }
        _ => unreachable!(),
    }
}

#[tokio::test]
async fn get_all_returns_all_ingredients() {
    let repo = InMemoryIngredientRepository::new();
    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets(vec![]),
    })
    .await
    .unwrap();

    repo.insert(Ingredient {
        id: Uuid::from_u128(2),
        name: "Ingredient name 2".try_into().unwrap(),
        description: "Ingredient description 2".try_into().unwrap(),
        diet_friendly: WhichDiets(vec![]),
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
        diet_friendly: WhichDiets(vec![]),
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
        diet_friendly: WhichDiets(vec![]),
    };
    repo.insert(input.clone()).await.unwrap();

    let error = repo
        .update(input.id, IngredientChangeset::default())
        .await
        .unwrap_err();

    match error {
        IngredientRepositoryError::ValidationError(ValidationError::EmptyField(fields)) => {
            assert_eq!(fields, vec!["name", "description", "diet_friendly"]);
        }
        _ => unreachable!(),
    };
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

    match error {
        IngredientRepositoryError::NotFound(u) => {
            assert_eq!(u, Uuid::from_u128(1))
        }
        _ => unreachable!(),
    }
}
