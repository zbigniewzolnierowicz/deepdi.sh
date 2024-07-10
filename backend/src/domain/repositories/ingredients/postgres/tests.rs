use sqlx::PgPool;

use super::*;
use crate::domain::{entities::ingredient::types::WhichDiets, repositories::ingredients::__test__};

use pretty_assertions::assert_eq;

#[sqlx::test]
async fn insert_ingredient_succeeds(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool.clone());

    repo.insert(Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
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
    let repo = PostgresIngredientRepository::new(pool);
    __test__::insert_ingredient_that_already_exists_fails_id(repo).await
}

#[sqlx::test]
async fn insert_ingredient_that_already_exists_fails_name(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool);
    __test__::insert_ingredient_that_already_exists_fails_name(repo).await
}

#[sqlx::test]
async fn get_by_id_returns_ingredient(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool);
    __test__::get_by_id_returns_ingredient(repo).await
}

#[sqlx::test]
async fn get_by_id_returns_error_when_missing(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool);
    __test__::get_by_id_returns_error_when_missing(repo).await
}

#[sqlx::test]
async fn get_all_returns_all_ingredients(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool);
    __test__::get_all_returns_all_ingredients(repo).await
}

#[sqlx::test]
async fn get_all_returns_empty_vec(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool);
    __test__::get_all_returns_empty_vec(repo).await
}

#[sqlx::test]
async fn deleting_works(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool);
    __test__::deleting_works(repo).await
}

#[sqlx::test]
async fn deleting_nonexistent_ingredient_errors(pool: PgPool) {
    let repo = PostgresIngredientRepository::new(pool);
    __test__::deleting_works(repo).await
}
