use sqlx::PgPool;

use super::*;
use crate::domain::repositories::ingredients::__test__;

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
