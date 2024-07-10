use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use crate::domain::commands::recipes::update::{update_recipe, UpdateRecipe, UpdateRecipeError};
use crate::domain::entities::recipe::{Recipe, ServingsType};
use crate::domain::repositories::ingredients::IngredientRepository;

use crate::domain::repositories::recipe::{RecipeRepository, RecipeRepositoryService};
use crate::test_utils::{insert_all_ingredients_of_recipe, recipe_changeset, recipe_fixture};

pub async fn updating_a_recipe_succeeds(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));
    let recipe = recipe_fixture();
    let changeset = recipe_changeset();
    insert_all_ingredients_of_recipe(ingredient_repo, &recipe).await;

    recipe_repo.insert(recipe.clone()).await.unwrap();

    let result = update_recipe(recipe_repo, &recipe.id, changeset)
        .await
        .unwrap();

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

pub async fn updating_a_nonexistent_recipe_fails(repo: impl RecipeRepository) {
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));

    let recipe = recipe_fixture();
    let changeset = UpdateRecipe {
        name: Some("WE UPDATED THIS THING".to_string()),
        ..Default::default()
    };

    let result = update_recipe(recipe_repo, &recipe.id, changeset)
        .await
        .unwrap_err();

    assert!(matches!(result, UpdateRecipeError::NotFound(id) if id == recipe.id))
}

pub async fn updating_a_recipe_with_empty_changeset_errors(
    repo: impl RecipeRepository,
    ingredient_repo: impl IngredientRepository,
) {
    let recipe_repo: RecipeRepositoryService = Arc::new(Box::new(repo));

    let recipe = recipe_fixture();
    let changeset = UpdateRecipe {
        ..Default::default()
    };
    insert_all_ingredients_of_recipe(ingredient_repo, &recipe).await;
    recipe_repo.insert(recipe.clone()).await.unwrap();

    let result = update_recipe(recipe_repo, &recipe.id, changeset)
        .await
        .unwrap_err();

    assert!(matches!(result, UpdateRecipeError::ChangesetEmpty))
}
