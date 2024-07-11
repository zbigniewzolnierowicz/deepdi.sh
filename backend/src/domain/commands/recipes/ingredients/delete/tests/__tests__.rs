use crate::domain::{
    commands::recipes::ingredients::delete::{
        delete_ingredient_from_recipe, DeleteIngredientFromRecipeError,
    },
    repositories::recipe::RecipeRepositoryService,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        entities::recipe::{IngredientWithAmount, Recipe},
        repositories::{ingredients::IngredientRepository, recipe::RecipeRepository},
    },
    test_utils::{ingredient_fixture, insert_all_ingredients_of_recipe, recipe_fixture},
};

pub async fn deleting_an_existing_ingredient_works(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let initial_recipe = recipe_fixture();
    insert_all_ingredients_of_recipe(&ingredient_repo, &initial_recipe).await;
    repo.insert(initial_recipe.clone()).await.unwrap();

    let ingredient_to_delete = initial_recipe.ingredients.first().unwrap();

    let repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    delete_ingredient_from_recipe(
        repo.clone(),
        &initial_recipe.id,
        &ingredient_to_delete.ingredient.id,
    )
    .await
    .unwrap();

    let recipe = repo.get_by_id(&initial_recipe.id).await.unwrap();

    assert!(recipe.ingredients.len() < initial_recipe.ingredients.len())
}

pub async fn deleting_an_ingredient_that_doesnt_appear_in_recipe_errors(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let initial_recipe = recipe_fixture();
    insert_all_ingredients_of_recipe(&ingredient_repo, &initial_recipe).await;
    repo.insert(initial_recipe.clone()).await.unwrap();

    let repo: RecipeRepositoryService = Arc::new(Box::new(repo));

    let error =
        delete_ingredient_from_recipe(repo.clone(), &initial_recipe.id, &Uuid::from_u128(999))
            .await
            .unwrap_err();

    assert!(
        matches!(error, DeleteIngredientFromRecipeError::RecipeHasNoIngredientError(id) if id == Uuid::from_u128(999))
    )
}

pub async fn deleting_an_ingredient_in_recipe_that_doesnt_exist_errors(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let initial_recipe = recipe_fixture();
    insert_all_ingredients_of_recipe(&ingredient_repo, &initial_recipe).await;

    let repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    let error = delete_ingredient_from_recipe(repo.clone(), &initial_recipe.id, &Uuid::nil())
        .await
        .unwrap_err();

    assert!(
        matches!(error, DeleteIngredientFromRecipeError::RecipeNotFoundError(id) if id == initial_recipe.id)
    )
}

pub async fn deleting_the_last_ingredient_in_recipe_errors(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let ingredient = IngredientWithAmount {
        ingredient: ingredient_fixture(),
        amount: crate::domain::entities::recipe::IngredientUnit::Grams(10.0),
        notes: None,
        optional: false,
    };

    let initial_recipe = Recipe {
        ingredients: vec![ingredient.clone()].try_into().unwrap(),
        ..recipe_fixture()
    };

    insert_all_ingredients_of_recipe(&ingredient_repo, &initial_recipe).await;

    repo.insert(initial_recipe.clone()).await.unwrap();

    let ingredient_to_delete = initial_recipe.ingredients.first().unwrap();

    let repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    let error = delete_ingredient_from_recipe(
        repo.clone(),
        &initial_recipe.id,
        &ingredient_to_delete.ingredient.id,
    )
    .await
    .unwrap_err();

    assert!(matches!(
        error,
        DeleteIngredientFromRecipeError::LastIngredientError
    ))
}
