use super::*;
use crate::domain::{entities::ingredient::types::WhichDiets, repositories::ingredients::__test__};
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
    __test__::insert_ingredient_that_already_exists_fails_id(repo).await
}

#[tokio::test]
async fn insert_ingredient_that_already_exists_fails_name() {
    let repo = InMemoryIngredientRepository::new();
    __test__::insert_ingredient_that_already_exists_fails_name(repo).await
}

#[tokio::test]
async fn get_by_id_returns_ingredient() {
    let repo = InMemoryIngredientRepository::new();
    __test__::get_by_id_returns_ingredient(repo).await
}

#[tokio::test]
async fn get_by_id_returns_error_when_missing() {
    let repo = InMemoryIngredientRepository::new();
    __test__::get_by_id_returns_error_when_missing(repo).await
}

#[tokio::test]
async fn get_all_returns_all_ingredients() {
    let repo = InMemoryIngredientRepository::new();
    __test__::get_all_returns_all_ingredients(repo).await
}

#[tokio::test]
async fn get_all_returns_empty_vec() {
    let repo = InMemoryIngredientRepository::new();
    __test__::get_all_returns_empty_vec(repo).await
}

#[tokio::test]
async fn deleting_works() {
    let repo = InMemoryIngredientRepository::new();
    __test__::deleting_works(repo).await
}

#[tokio::test]
async fn deleting_nonexistent_ingredient_errors() {
    let repo = InMemoryIngredientRepository::new();
    __test__::deleting_works(repo).await
}
