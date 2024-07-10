use uuid::Uuid;

use crate::{
    domain::{
        commands::recipes::ingredients::update::{
            update_ingredient_in_recipe, UpdateIngredientInRecipeError,
        },
        entities::recipe::IngredientUnit,
        repositories::{
            ingredients::IngredientRepository,
            recipe::{errors::GetRecipeByIdError, RecipeRepository, RecipeRepositoryService},
        },
    },
    test_utils::{insert_all_ingredients_of_recipe, recipe_fixture},
};
use std::sync::Arc;

pub async fn updating_ingredient_in_recipe_works(
    recipe_repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let initial_recipe = recipe_fixture();
    insert_all_ingredients_of_recipe(&ingredient_repo, &initial_recipe).await;
    recipe_repo.insert(initial_recipe.clone()).await.unwrap();

    let ingredient_to_update = initial_recipe.ingredients.first().unwrap();
    let amount = IngredientUnit::Cups(2.0);

    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(recipe_repo));

    let result = update_ingredient_in_recipe(
        recipe_repo,
        &initial_recipe.id,
        &ingredient_to_update.ingredient.id,
        amount.clone(),
    )
    .await
    .unwrap();

    assert_eq!(result.ingredients.first().unwrap().amount, amount);
}

pub async fn updating_ingredient_in_nonexistent_recipe_errors(
    recipe_repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let initial_recipe = recipe_fixture();
    insert_all_ingredients_of_recipe(&ingredient_repo, &initial_recipe).await;

    let ingredient_to_update = initial_recipe.ingredients.first().unwrap();
    let amount = IngredientUnit::Cups(2.0);

    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(recipe_repo));

    let error = update_ingredient_in_recipe(
        recipe_repo,
        &initial_recipe.id,
        &ingredient_to_update.ingredient.id,
        amount.clone(),
    )
    .await
    .unwrap_err();

    assert!(
        matches!(error, UpdateIngredientInRecipeError::GetRecipe(GetRecipeByIdError::NotFound(id)) if id == initial_recipe.id)
    )
}

pub async fn updating_nonexistent_ingredient_in_recipe_errors(
    recipe_repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let initial_recipe = recipe_fixture();
    insert_all_ingredients_of_recipe(&ingredient_repo, &initial_recipe).await;
    recipe_repo.insert(initial_recipe.clone()).await.unwrap();

    let amount = IngredientUnit::Cups(2.0);

    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(recipe_repo));

    let error = update_ingredient_in_recipe(
        recipe_repo,
        &initial_recipe.id,
        &Uuid::from_u128(0xff),
        amount.clone(),
    )
    .await
    .unwrap_err();

    assert!(
        matches!(error, UpdateIngredientInRecipeError::MissingIngredient(id) if id == Uuid::from_u128(0xff))
    )
}
