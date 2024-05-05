use crate::test_utils::recipe_fixture;

use super::*;

#[tokio::test]
async fn creating_recipe_works() {
    let repo = InMemoryRecipeRepository::new();

    let recipe = recipe_fixture();
    let result = repo.insert(recipe.clone()).await.unwrap();
    assert_eq!(recipe, result);

    let lock = repo.0.lock().unwrap();
    let inner_result = lock
        .get(&result.id)
        .expect("The recipe wasn't found in the hashmap");

    assert_eq!(inner_result, &recipe.clone());
}

#[tokio::test]
async fn inserting_recipe_with_same_id_fails() {
    let repo = InMemoryRecipeRepository::new();

    let recipe = recipe_fixture();

    repo.insert(recipe.clone()).await.unwrap();

    let error = repo.insert(recipe.clone()).await.unwrap_err();

    assert!(matches!(error, RecipeRepositoryError::Conflict(a) if a == "id"));
}
