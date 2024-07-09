use thiserror::Error;
use uuid::Uuid;

use crate::domain::{
    entities::recipe::{IngredientUnit, Recipe},
    repositories::recipe::{
        errors::{
            GetRecipeByIdError,
            UpdateIngredientInRecipeError as UpdateIngredientInRecipeErrorInternal,
        },
        RecipeRepositoryService,
    },
};

#[derive(Error, Debug, strum::AsRefStr)]
pub enum UpdateIngredientInRecipeError {
    #[error(transparent)]
    GetRecipe(#[from] GetRecipeByIdError),

    #[error("Could not find ingredient with ID {0} in this recipe.")]
    MissingIngredient(Uuid),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl From<UpdateIngredientInRecipeErrorInternal> for UpdateIngredientInRecipeError {
    fn from(value: UpdateIngredientInRecipeErrorInternal) -> Self {
        match value {
            UpdateIngredientInRecipeErrorInternal::RecipeHasNoIngredientError(id) => {
                Self::MissingIngredient(id)
            }
            e => e.into(),
        }
    }
}

pub async fn update_ingredient_in_recipe(
    recipe_repo: RecipeRepositoryService,
    recipe_id: &Uuid,
    ingredient_id: &Uuid,
    amount: IngredientUnit,
) -> Result<Recipe, UpdateIngredientInRecipeError> {
    let recipe = recipe_repo.get_by_id(recipe_id).await?;

    let ingredient_in_recipe = &recipe
        .ingredients
        .iter()
        .find(|x| x.ingredient.id == *ingredient_id)
        .ok_or_else(|| UpdateIngredientInRecipeError::MissingIngredient(*ingredient_id))?;

    recipe_repo
        .update_ingredient_amount(&recipe, &ingredient_in_recipe, &amount)
        .await?;

    let recipe = recipe_repo.get_by_id(recipe_id).await?;

    Ok(recipe)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod __tests__ {
        use std::sync::Arc;

        use crate::{
            domain::repositories::{
                ingredients::IngredientRepository,
                recipe::{RecipeRepository, __test__::insert_all_ingredients_of_recipe},
            },
            test_utils::recipe_fixture,
        };

        use super::*;

        pub async fn updating_ingredient_in_recipe_works(
            recipe_repo: impl RecipeRepository,
            ingredient_repo: impl IngredientRepository,
        ) {
            let initial_recipe = recipe_fixture();
            insert_all_ingredients_of_recipe(ingredient_repo, &initial_recipe).await;
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
            insert_all_ingredients_of_recipe(ingredient_repo, &initial_recipe).await;

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
            insert_all_ingredients_of_recipe(ingredient_repo, &initial_recipe).await;
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
    }

    mod in_memory {
        use super::__tests__;
        use crate::domain::repositories::{
            ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
        };

        #[tokio::test]
        async fn updating_ingredient_in_recipe_works() {
            let recipe_repo = InMemoryRecipeRepository::new();
            let ingredient_repo = InMemoryIngredientRepository::new();

            __tests__::updating_ingredient_in_recipe_works(recipe_repo, ingredient_repo).await
        }

        #[tokio::test]
        async fn updating_ingredient_in_nonexistent_recipe_errors() {
            let recipe_repo = InMemoryRecipeRepository::new();
            let ingredient_repo = InMemoryIngredientRepository::new();

            __tests__::updating_ingredient_in_nonexistent_recipe_errors(
                recipe_repo,
                ingredient_repo,
            )
            .await
        }

        #[tokio::test]
        async fn updating_nonexistent_ingredient_in_recipe_errors() {
            let recipe_repo = InMemoryRecipeRepository::new();
            let ingredient_repo = InMemoryIngredientRepository::new();

            __tests__::updating_nonexistent_ingredient_in_recipe_errors(
                recipe_repo,
                ingredient_repo,
            )
            .await
        }
    }

    mod sql {
        use sqlx::PgPool;

        use crate::domain::repositories::{
            ingredients::postgres::PostgresIngredientRepository,
            recipe::postgres::PostgresRecipeRepository,
        };

        use super::__tests__;

        #[sqlx::test]
        async fn updating_ingredient_in_recipe_works(pool: PgPool) {
            let recipe_repo = PostgresRecipeRepository::new(pool.clone());
            let ingredient_repo = PostgresIngredientRepository::new(pool.clone());

            __tests__::updating_ingredient_in_recipe_works(recipe_repo, ingredient_repo).await
        }

        #[sqlx::test]
        async fn updating_ingredient_in_nonexistent_recipe_errors(pool: PgPool) {
            let recipe_repo = PostgresRecipeRepository::new(pool.clone());
            let ingredient_repo = PostgresIngredientRepository::new(pool.clone());

            __tests__::updating_ingredient_in_nonexistent_recipe_errors(
                recipe_repo,
                ingredient_repo,
            )
            .await
        }

        #[sqlx::test]
        async fn updating_nonexistent_ingredient_in_recipe_errors(pool: PgPool) {
            let recipe_repo = PostgresRecipeRepository::new(pool.clone());
            let ingredient_repo = PostgresIngredientRepository::new(pool.clone());

            __tests__::updating_nonexistent_ingredient_in_recipe_errors(
                recipe_repo,
                ingredient_repo,
            )
            .await
        }
    }
}
