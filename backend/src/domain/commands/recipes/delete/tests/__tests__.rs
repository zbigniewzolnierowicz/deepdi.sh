use std::sync::Arc;

use crate::{
    domain::{
        commands::recipes::delete::{delete_recipe, DeleteRecipeError},
        repositories::{
            ingredients::IngredientRepository,
            recipe::{RecipeRepository, RecipeRepositoryService},
        },
    },
    test_utils::{insert_all_ingredients_of_recipe, recipe_fixture},
};

pub async fn deleting_a_recipe_succeeds(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    let recipe = recipe_fixture();

    insert_all_ingredients_of_recipe(&ingredient_repo, &recipe).await;

    let result = repo.insert(recipe.clone()).await.unwrap();

    delete_recipe(repo, &result.id).await.unwrap();
}

pub async fn deleting_a_nonexistent_recipe_fails(repo: impl RecipeRepository) {
    let repo: RecipeRepositoryService = Arc::new(Box::new(repo));

    let recipe = recipe_fixture();
    let result = delete_recipe(repo, &recipe.id).await.unwrap_err();

    assert!(matches!(result, DeleteRecipeError::NotFound(id) if id == recipe.id))
}
