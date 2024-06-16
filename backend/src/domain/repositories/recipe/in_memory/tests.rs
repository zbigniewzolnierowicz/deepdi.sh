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

    dbg!(&error);

    assert!(matches!(error, InsertRecipeError::Conflict(a) if a == "recipe id"));
}

#[tokio::test]
async fn getting_a_recipe_by_id_succeeds() {
    let repo = InMemoryRecipeRepository::new();
    let recipe = recipe_fixture();
    let result = repo.insert(recipe.clone()).await.unwrap();
    let result = repo.get_by_id(&result.id).await.unwrap();

    assert_eq!(recipe, result);
}

#[tokio::test]
async fn getting_a_nonexistent_recipe_by_id_fails() {
    let repo = InMemoryRecipeRepository::new();
    let recipe = recipe_fixture();
    let result = repo.get_by_id(&recipe.id).await.unwrap_err();

    assert!(matches!(result, GetRecipeByIdError::NotFound(id) if id == recipe.id));
}

#[tokio::test]
async fn deleting_a_recipe_succeeds() {
    let repo = InMemoryRecipeRepository::new();
    let recipe = recipe_fixture();
    let result = repo.insert(recipe.clone()).await.unwrap();
    repo.delete(&result.id).await.unwrap();
}

#[tokio::test]
async fn deleting_a_nonexistent_recipe_fails() {
    let repo = InMemoryRecipeRepository::new();
    let recipe = recipe_fixture();
    let result = repo.delete(&recipe.id).await.unwrap_err();

    assert!(matches!(result, DeleteRecipeError::NotFound(id) if id == recipe.id))
}

#[tokio::test]
async fn updating_a_recipe_succeeds() {
    let repo = InMemoryRecipeRepository::new();
    let recipe = recipe_fixture();
    let changeset = RecipeChangeset {
        name: Some("WE UPDATED THIS THING".to_string()),
        ..Default::default()
    };
    let result = repo.insert(recipe.clone()).await.unwrap();
    let result = repo.update(&result.id, changeset).await.unwrap();

    assert_eq!(&result.name, "WE UPDATED THIS THING");

    let result = repo.get_by_id(&result.id).await.unwrap();

    assert_eq!(&result.name, "WE UPDATED THIS THING");
}

#[tokio::test]
async fn updating_a_nonexistent_recipe_fails() {
    let repo = InMemoryRecipeRepository::new();
    let recipe = recipe_fixture();
    let changeset = RecipeChangeset {
        name: Some("WE UPDATED THIS THING".to_string()),
        ..Default::default()
    };
    let result = repo.update(&recipe.id, changeset).await.unwrap_err();

    assert!(
        matches!(result, UpdateRecipeError::Get(GetRecipeByIdError::NotFound(id)) if id == recipe.id)
    )
}

#[tokio::test]
async fn updating_a_recipe_with_empty_changeset_does_nothing() {
    let repo = InMemoryRecipeRepository::new();
    let recipe = recipe_fixture();
    let changeset = RecipeChangeset {
        ..Default::default()
    };
    let result = repo.insert(recipe.clone()).await.unwrap();
    let result = repo.update(&result.id, changeset).await.unwrap();

    assert_eq!(recipe, result);
}
