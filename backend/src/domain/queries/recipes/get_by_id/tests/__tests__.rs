use crate::domain::queries::recipes::get_by_id::GetRecipeError;
use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::{
        queries::recipes::get_by_id::get_recipe_by_id,
        repositories::{
            ingredients::IngredientRepository,
            recipe::{RecipeRepository, RecipeRepositoryService},
        },
    },
    test_utils::{insert_all_ingredients_of_recipe, recipe_fixture},
};

pub async fn getting_recipe_by_id_works(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients_of_recipe(&ingredient_repo, &recipe).await;

    repo.insert(recipe.clone()).await.unwrap();

    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    let result = get_recipe_by_id(recipe_repo, &recipe.id).await.unwrap();

    assert_eq!(result, recipe);
}

pub async fn getting_a_nonexistent_recipe_errors(repo: impl RecipeRepository) {
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    let error = get_recipe_by_id(recipe_repo, &Uuid::nil())
        .await
        .unwrap_err();

    assert!(matches!(error, GetRecipeError::NotFound(id) if id == Uuid::nil()));
}
