use pretty_assertions::assert_eq;
use std::{
    collections::{BTreeMap, HashSet},
    time::Duration,
};

use uuid::Uuid;

use crate::{
    domain::{
        entities::recipe::{IngredientUnit, IngredientWithAmount},
        repositories::{
            ingredients::IngredientRepository,
            recipe::errors::{DeleteRecipeError, GetRecipeByIdError, InsertRecipeError},
        },
    },
    test_utils::{ingredient_fixture, insert_all_ingredients_of_recipe, recipe_fixture},
};

use super::RecipeRepository;
pub async fn creating_recipe_works(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients_of_recipe(ingredient_repo, &recipe).await;

    let result = repo.insert(recipe.clone()).await.unwrap();
    assert_eq!(recipe, result);
}

pub async fn inserting_recipe_with_same_id_fails(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients_of_recipe(ingredient_repo, &recipe).await;

    repo.insert(recipe.clone()).await.unwrap();

    let error = repo.insert(recipe.clone()).await.unwrap_err();

    assert!(matches!(error, InsertRecipeError::Conflict(a) if a == "recipe id"));
}

pub async fn getting_recipe_by_id_works(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients_of_recipe(ingredient_repo, &recipe).await;

    repo.insert(recipe.clone()).await.unwrap();

    let result = repo.get_by_id(&recipe.id).await.unwrap();

    assert_eq!(result, recipe);
}

pub async fn getting_a_nonexistent_recipe_errors(repo: impl RecipeRepository) {
    let error = repo.get_by_id(&Uuid::nil()).await.unwrap_err();

    assert!(matches!(error, GetRecipeByIdError::NotFound(id) if id == Uuid::nil()));
}

pub async fn deleting_a_recipe_succeeds(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients_of_recipe(ingredient_repo, &recipe).await;

    let result = repo.insert(recipe.clone()).await.unwrap();

    repo.delete(&result.id).await.unwrap();
}

pub async fn deleting_a_nonexistent_recipe_fails(repo: impl RecipeRepository) {
    let recipe = recipe_fixture();
    let result = repo.delete(&recipe.id).await.unwrap_err();

    assert!(matches!(result, DeleteRecipeError::NotFound(id) if id == recipe.id))
}
