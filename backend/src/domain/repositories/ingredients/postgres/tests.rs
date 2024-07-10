use sqlx::PgPool;

use super::*;
use crate::domain::repositories::ingredients::__test__;

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
