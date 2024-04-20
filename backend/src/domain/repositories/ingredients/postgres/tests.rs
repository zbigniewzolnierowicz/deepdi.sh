use sqlx::PgPool;

use super::*;
use crate::domain::entities::ingredient::types::{
    IngredientDescription, IngredientName, WhichDiets,
};

use pretty_assertions::assert_eq;

#[sqlx::test]
async fn insert_ingredient_succeeds(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());

    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets(vec![]),
    })
    .await
    .unwrap();

    let ingredient = sqlx::query_as!(
        IngredientModel,
        "SELECT id, name, description, diet_friendly FROM ingredients WHERE id = $1",
        Uuid::from_u128(1)
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(ingredient.len(), 1);
}

#[sqlx::test]
async fn insert_ingredient_that_already_exists_fails_id(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());

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

#[sqlx::test]
async fn insert_ingredient_that_already_exists_fails_name(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());

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

#[sqlx::test]
async fn get_by_id_returns_ingredient(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());
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

#[sqlx::test]
async fn get_by_id_returns_error_when_missing(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());

    let result = repo.get_by_id(Uuid::from_u128(1)).await.unwrap_err();

    match result {
        IngredientRepositoryError::NotFound(id) => {
            assert_eq!(id, Uuid::from_u128(1));
        }
        _ => unreachable!(),
    }
}

#[sqlx::test]
async fn get_all_returns_all_ingredients(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());
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

    let result = repo.get_all().await.unwrap();

    assert_eq!(result.len(), 2);
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

#[sqlx::test]
async fn get_all_returns_empty_vec(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());
    let result = repo.get_all().await.unwrap();

    assert_eq!(result, vec![]);
}

// TODO: add tests for updates
