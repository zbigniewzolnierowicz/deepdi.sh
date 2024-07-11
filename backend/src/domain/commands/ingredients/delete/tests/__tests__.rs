use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::{
        commands::ingredients::delete::{delete_ingredient, DeleteIngredientError},
        entities::ingredient::{types::WhichDiets, Ingredient},
        repositories::{
            ingredients::{IngredientRepository, IngredientRepositoryService},
            recipe::{RecipeRepository, RecipeRepositoryService},
        },
    },
    test_utils::{ingredient_fixture, insert_all_ingredients_of_recipe, recipe_fixture},
};

pub async fn deleting_works(repo: impl IngredientRepository, recipe_repo: impl RecipeRepository) {
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(recipe_repo));
    let input = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    };

    let insert_result = repo.insert(input).await.unwrap();
    delete_ingredient(repo, recipe_repo, &insert_result.id)
        .await
        .unwrap();
}

pub async fn deleting_nonexistent_ingredient_errors(
    repo: impl IngredientRepository,
    recipe_repo: impl RecipeRepository,
) {
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(recipe_repo));
    let ingredient = ingredient_fixture();
    let error = delete_ingredient(repo, recipe_repo, &ingredient.id)
        .await
        .unwrap_err();

    assert!(matches!(
        error,
        DeleteIngredientError::NotFound(id) if id == ingredient.id
    ));
}

pub async fn deleting_an_ingredient_still_in_use_by_recipes_errors(
    repo: impl IngredientRepository,
    recipe_repo: impl RecipeRepository,
) {
    let recipe = recipe_fixture();
    insert_all_ingredients_of_recipe(&repo, &recipe).await;
    recipe_repo.insert(recipe.clone()).await.unwrap();
    let input = &recipe.ingredients.first().unwrap().ingredient.id;

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(recipe_repo));

    let error = delete_ingredient(repo, recipe_repo, input)
        .await
        .unwrap_err();

    assert!(matches!(error, DeleteIngredientError::InUseByRecipe));
}
