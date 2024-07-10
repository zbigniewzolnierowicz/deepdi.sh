use std::{collections::HashSet, sync::Arc};

use crate::{
    domain::{
        commands::recipes::ingredients::add::add_ingredient_to_recipe, entities::recipe::{IngredientAmountData, IngredientUnit, IngredientWithAmount}, repositories::{ingredients::{IngredientRepository, IngredientRepositoryService}, recipe::{RecipeRepository, RecipeRepositoryService}}
    },
    test_utils::{ingredient_fixture, insert_all_ingredients, insert_all_ingredients_of_recipe, recipe_fixture},
};

pub async fn adding_an_ingredient_to_a_recipe_works(
    repo: impl RecipeRepository,
    ing_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();
    let ingredient = IngredientWithAmount {
        ingredient: ingredient_fixture(),
        amount: IngredientUnit::Grams(666.0),
        notes: None,
        optional: true,
    };
    let mut all_ingredients = recipe.ingredients.to_vec().clone();
    all_ingredients.push(ingredient.clone());

    insert_all_ingredients(&ing_repo, &all_ingredients).await;

    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    let ingredient_repo: IngredientRepositoryService = Arc::new(Box::new(ing_repo));

    let ingredient_payload = IngredientAmountData::from(ingredient.clone());

    let recipe = recipe_repo.insert(recipe.clone()).await.unwrap();

    let updated_recipe = add_ingredient_to_recipe(recipe_repo, ingredient_repo, &recipe.id, ingredient_payload).await.unwrap();

    let expected: HashSet<_> = all_ingredients
        .iter()
        .map(|item| item.ingredient.id)
        .collect();

    assert!(updated_recipe
        .ingredients
        .iter()
        .all(|item| expected.contains(&item.ingredient.id)));
}

pub async fn adding_a_nonexistent_ingredient_to_a_recipe_errors(
    recipe_repo: impl RecipeRepository,
    ing_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();
    let ingredient = IngredientWithAmount {
        ingredient: ingredient_fixture(),
        amount: IngredientUnit::Grams(666.0),
        notes: None,
        optional: true,
    };

    insert_all_ingredients_of_recipe(&ing_repo, &recipe).await;
    let recipe = recipe_repo.insert(recipe.clone()).await.unwrap();

    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(recipe_repo));
    let ingredient_repo: IngredientRepositoryService = Arc::new(Box::new(ing_repo));

    let ingredient_payload = IngredientAmountData::from(ingredient.clone());
}
