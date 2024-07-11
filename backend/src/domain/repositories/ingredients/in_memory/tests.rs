use super::*;
use crate::domain::repositories::ingredients::__test__;

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
