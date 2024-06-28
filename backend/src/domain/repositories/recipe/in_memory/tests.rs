use crate::domain::repositories::{ingredients::InMemoryIngredientRepository, recipe::__test__};

use super::*;

#[tokio::test]
async fn creating_recipe_works() {
    let repo = InMemoryRecipeRepository::new();
    let ingredient_repo = InMemoryIngredientRepository::new();
    __test__::creating_recipe_works(repo, ingredient_repo).await
}

#[tokio::test]
async fn inserting_recipe_with_same_id_fails() {
    let repo = InMemoryRecipeRepository::new();
    let ingredient_repo = InMemoryIngredientRepository::new();
    __test__::inserting_recipe_with_same_id_fails(repo, ingredient_repo).await
}

#[tokio::test]
async fn getting_recipe_by_id_works() {
    let repo = InMemoryRecipeRepository::new();
    let ingredient_repo = InMemoryIngredientRepository::new();
    __test__::getting_recipe_by_id_works(repo, ingredient_repo).await
}

#[tokio::test]
async fn tokio_getting_a_nonexistent_recipe_errors() {
    let repo = InMemoryRecipeRepository::new();
    __test__::getting_a_nonexistent_recipe_errors(repo).await
}

#[tokio::test]
async fn deleting_a_recipe_succeeds() {
    let repo = InMemoryRecipeRepository::new();
    let ingredient_repo = InMemoryIngredientRepository::new();
    __test__::deleting_a_recipe_succeeds(repo, ingredient_repo).await
}

#[tokio::test]
async fn deleting_a_nonexistent_recipe_fails() {
    let repo = InMemoryRecipeRepository::new();
    __test__::deleting_a_nonexistent_recipe_fails(repo).await
}

#[tokio::test]
async fn updating_a_recipe_succeeds() {
    let ingredient_repo = InMemoryIngredientRepository::new();
    let repo = InMemoryRecipeRepository::new();
    __test__::updating_a_recipe_succeeds(repo, ingredient_repo).await
}

#[tokio::test]
async fn updating_a_nonexistent_recipe_fails() {
    let repo = InMemoryRecipeRepository::new();
    __test__::updating_a_nonexistent_recipe_fails(repo).await
}

#[tokio::test]
async fn updating_a_recipe_with_empty_changeset_does_nothing() {
    let ingredient_repo = InMemoryIngredientRepository::new();
    let repo = InMemoryRecipeRepository::new();
    __test__::updating_a_recipe_with_empty_changeset_does_nothing(repo, ingredient_repo).await
}
