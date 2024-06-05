use crate::{
    domain::repositories::ingredients::{
        postgres::PostgresIngredientRepository, IngredientRepository,
    },
    test_utils::recipe_fixture,
};

use super::*;

#[sqlx::test]
async fn creating_recipe_works(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool);

    let recipe = recipe_fixture();
    for ir in recipe.ingredients.clone() {
        ingredient_repo
            .insert(ir.ingredient)
            .await
            .expect("Could not insert an ingredient due to an error somewhere.");
    }
    let result = repo.insert(recipe.clone()).await.unwrap();
    assert_eq!(recipe, result);
}

#[sqlx::test]
async fn inserting_recipe_with_same_id_fails(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool.clone());

    let recipe = recipe_fixture();
    for ir in recipe.ingredients.clone() {
        ingredient_repo
            .insert(ir.ingredient)
            .await
            .expect("Could not insert an ingredient due to an error somewhere.");
    }

    repo.insert(recipe.clone()).await.unwrap();

    let error = repo.insert(recipe.clone()).await.unwrap_err();

    assert!(matches!(error, InsertRecipeError::Conflict(a) if a == "recipe id"));
}

#[sqlx::test]
async fn getting_recipe_by_id_works(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool);

    let recipe = recipe_fixture();
    for ir in recipe.ingredients.clone() {
        ingredient_repo
            .insert(ir.ingredient)
            .await
            .expect("Could not insert an ingredient due to an error somewhere.");
    }

    repo.insert(recipe.clone()).await.unwrap();

    let result = repo.get_by_id(&recipe.id).await.unwrap();

    assert_eq!(result, recipe);
}

#[sqlx::test]
async fn getting_a_nonexistent_recipe_errors(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let error = repo.get_by_id(&Uuid::nil()).await.unwrap_err();

    assert!(matches!(error, GetRecipeByIdError::NotFound(id) if id == Uuid::nil()));
}

#[sqlx::test]
async fn deleting_a_recipe_succeeds(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let recipe = recipe_fixture();
    let result = repo.insert(recipe.clone()).await.unwrap();
    repo.delete(&result.id).await.unwrap();
}

#[sqlx::test]
async fn deleting_a_nonexistent_recipe_fails(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let recipe = recipe_fixture();
    let result = repo.delete(&recipe.id).await.unwrap_err();

    assert!(matches!(result, DeleteRecipeError::NotFound(id) if id == recipe.id))
}
