use super::*;
use crate::domain::repositories::ingredients::__test__;

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
