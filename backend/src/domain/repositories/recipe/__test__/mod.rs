use std::{collections::BTreeMap, time::Duration};
use pretty_assertions::assert_eq;

use futures::future::join_all;
use uuid::Uuid;

use crate::{
    domain::{
        entities::recipe::{Recipe, RecipeChangeset, ServingsType},
        repositories::{
            ingredients::IngredientRepository,
            recipe::errors::{DeleteRecipeError, GetRecipeByIdError, InsertRecipeError, UpdateRecipeError},
        },
    },
    test_utils::{recipe_changeset, recipe_fixture},
};

use super::RecipeRepository;

pub async fn insert_all_ingredients(ingredient_repo: impl IngredientRepository, recipe: &Recipe) {
    join_all(
        recipe
            .ingredients
            .as_ref()
            .iter()
            .map(|i| async { ingredient_repo.insert(i.ingredient.clone()).await.unwrap() }),
    )
    .await;
}

pub async fn creating_recipe_works(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients(ingredient_repo, &recipe).await;

    let result = repo.insert(recipe.clone()).await.unwrap();
    assert_eq!(recipe, result);
}

pub async fn inserting_recipe_with_same_id_fails(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients(ingredient_repo, &recipe).await;

    repo.insert(recipe.clone()).await.unwrap();

    let error = repo.insert(recipe.clone()).await.unwrap_err();

    assert!(matches!(error, InsertRecipeError::Conflict(a) if a == "recipe id"));
}

pub async fn getting_recipe_by_id_works(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();

    insert_all_ingredients(ingredient_repo, &recipe).await;

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

    insert_all_ingredients(ingredient_repo, &recipe).await;

    let result = repo.insert(recipe.clone()).await.unwrap();

    repo.delete(&result.id).await.unwrap();
}

pub async fn deleting_a_nonexistent_recipe_fails(repo: impl RecipeRepository) {
    let recipe = recipe_fixture();
    let result = repo.delete(&recipe.id).await.unwrap_err();

    assert!(matches!(result, DeleteRecipeError::NotFound(id) if id == recipe.id))
}

pub async fn updating_a_recipe_succeeds(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();
    let changeset = recipe_changeset();
    insert_all_ingredients(ingredient_repo, &recipe).await;

    let result = repo.insert(recipe.clone()).await.unwrap();
    repo.update(&result.id, changeset).await.unwrap();
    let result = repo.get_by_id(&result.id).await.unwrap();

    assert_eq!(
        result,
        Recipe {
            name: "WE UPDATED THIS THING".to_string(),
            description: "WE UPDATED THAT THING".to_string(),
            steps: vec!["WE UPDATED ANOTHER THING".to_string()]
                .try_into()
                .unwrap(),
            time: BTreeMap::from([("Prep time".to_string(), Duration::from_secs(60))]),
            servings: ServingsType::Exact(4),
            ..recipe
        }
    );
}

pub async fn updating_a_nonexistent_recipe_fails(
    repo: impl RecipeRepository,
) {
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

pub async fn updating_a_recipe_with_empty_changeset_does_nothing(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe = recipe_fixture();
    let changeset = RecipeChangeset {
        ..Default::default()
    };
    insert_all_ingredients(ingredient_repo, &recipe).await;
    let result = repo.insert(recipe.clone()).await.unwrap();
    let result = repo.update(&result.id, changeset).await.unwrap();

    assert_eq!(recipe, result);
}
