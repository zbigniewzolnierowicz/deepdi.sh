use crate::domain::repositories::{
    ingredients::postgres::PostgresIngredientRepository, recipe::__test__,
};

use super::*;

#[sqlx::test]
async fn creating_recipe_works(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool);

    __test__::creating_recipe_works(repo, ingredient_repo).await
}

#[sqlx::test]
async fn inserting_recipe_with_same_id_fails(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
    __test__::inserting_recipe_with_same_id_fails(repo, ingredient_repo).await
}

#[sqlx::test]
async fn getting_recipe_by_id_works(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool);
    __test__::getting_recipe_by_id_works(repo, ingredient_repo).await
}

#[sqlx::test]
async fn sqlx_getting_a_nonexistent_recipe_errors(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    __test__::getting_a_nonexistent_recipe_errors(repo).await
}

#[sqlx::test]
async fn deleting_a_recipe_succeeds(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
    __test__::deleting_a_recipe_succeeds(repo, ingredient_repo).await
}

#[sqlx::test]
async fn deleting_a_nonexistent_recipe_fails(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    __test__::deleting_a_nonexistent_recipe_fails(repo).await
}
