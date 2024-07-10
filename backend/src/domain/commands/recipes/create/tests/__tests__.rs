use std::sync::Arc;

use pretty_assertions::assert_eq;
use uuid::Uuid;

use crate::{
    domain::{
        commands::recipes::create::{create_recipe, CreateRecipeError},
        repositories::{
            ingredients::{IngredientRepository, IngredientRepositoryService},
            recipe::{RecipeRepository, RecipeRepositoryService},
        },
    },
    test_utils::{insert_all_ingredients_of_recipe, recipe_fixture},
};

pub async fn create_recipe_without_proper_ingredients_errors(
    repo: impl RecipeRepository,
    ing_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();
    let ingredient_repo: IngredientRepositoryService = Arc::new(Box::new(ing_repo));
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));

    let result = create_recipe(recipe_repo, ingredient_repo, &recipe.into())
        .await
        .unwrap_err();

    assert!(matches!(result, CreateRecipeError::IngredientsNotFound(_)));
}

pub async fn create_recipe_with_proper_ingredients(
    repo: impl RecipeRepository,
    ing_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients_of_recipe(&ing_repo, &recipe).await;

    let ingredient_repo: IngredientRepositoryService = Arc::new(Box::new(ing_repo));
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));

    let result = create_recipe(recipe_repo, ingredient_repo, &recipe.clone().into())
        .await
        .unwrap();

    assert_eq!(Uuid::get_version(&result.id), Some(uuid::Version::SortRand));
    assert_eq!(&result.name, "Hoisin Tofu and Broccoli");

    assert!(result
        .ingredients
        .iter()
        .all(|ing| recipe.ingredients.contains(ing)))
}
