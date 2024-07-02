use strum::AsRefStr;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::recipe::errors::ValidationError;
use crate::domain::repositories::recipe::errors::{
    DeleteIngredientFromRecipeError as DeleteIngredientFromRecipeErrorInternal, GetRecipeByIdError,
};
use crate::domain::repositories::recipe::RecipeRepositoryService;

#[derive(Error, Debug, AsRefStr)]
pub enum DeleteIngredientFromRecipeError {
    #[error("Could not found recipe with ID {0}")]
    RecipeNotFoundError(Uuid),

    #[error("The recipe has no ingredient with ID of {0}")]
    RecipeHasNoIngredientError(Uuid),

    #[error("There is only one ingredient in the recipe. A recipe should have one ingredient at minimum.")]
    LastIngredientError,

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl From<DeleteIngredientFromRecipeErrorInternal> for DeleteIngredientFromRecipeError {
    fn from(value: DeleteIngredientFromRecipeErrorInternal) -> Self {
        match value {
            DeleteIngredientFromRecipeErrorInternal::RecipeHasNoIngredientError(id) => {
                Self::RecipeHasNoIngredientError(id)
            }
            DeleteIngredientFromRecipeErrorInternal::ValidationError(
                ValidationError::EmptyField(field),
            ) if field == vec!["steps"] => Self::LastIngredientError,
            e => e.into(),
        }
    }
}

impl From<GetRecipeByIdError> for DeleteIngredientFromRecipeError {
    fn from(value: GetRecipeByIdError) -> Self {
        match value {
            GetRecipeByIdError::NotFound(id) => Self::RecipeNotFoundError(id),
            e => e.into(),
        }
    }
}

pub async fn delete_ingredient_from_recipe(
    recipe_repo: RecipeRepositoryService,
    recipe_id: &Uuid,
    ingredient_id: &Uuid,
) -> Result<(), DeleteIngredientFromRecipeError> {
    let recipe = recipe_repo.get_by_id(recipe_id).await?;

    if recipe.ingredients.len() == 1 {
        return Err(DeleteIngredientFromRecipeError::LastIngredientError);
    };

    let ingredient_in_recipe = &recipe
        .ingredients
        .iter()
        .find(|x| x.ingredient.id == *ingredient_id)
        .ok_or_else(|| {
            DeleteIngredientFromRecipeError::RecipeHasNoIngredientError(*ingredient_id)
        })?;

    recipe_repo
        .delete_ingredient(&recipe, &ingredient_in_recipe)
        .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: replace this with macros somehow
    // TODO: replace all other repo tests with command tests
    pub(super) mod __tests__ {

        use super::*;
        use std::sync::Arc;

        use crate::{
            domain::{
                entities::recipe::{IngredientWithAmount, Recipe},
                repositories::{
                    ingredients::IngredientRepository,
                    recipe::{RecipeRepository, __test__::insert_all_ingredients_of_recipe},
                },
            },
            test_utils::{ingredient_fixture, recipe_fixture},
        };

        pub async fn deleting_an_existing_ingredient_works(
            repo: impl RecipeRepository,
            ingredient_repo: impl IngredientRepository,
        ) {
            let initial_recipe = recipe_fixture();
            insert_all_ingredients_of_recipe(ingredient_repo, &initial_recipe).await;
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
            insert_all_ingredients_of_recipe(ingredient_repo, &initial_recipe).await;
            repo.insert(initial_recipe.clone()).await.unwrap();

            let repo: RecipeRepositoryService = Arc::new(Box::new(repo));

            let error = delete_ingredient_from_recipe(
                repo.clone(),
                &initial_recipe.id,
                &Uuid::from_u128(999),
            )
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
            insert_all_ingredients_of_recipe(ingredient_repo, &initial_recipe).await;

            let repo: RecipeRepositoryService = Arc::new(Box::new(repo));
            let error =
                delete_ingredient_from_recipe(repo.clone(), &initial_recipe.id, &Uuid::nil())
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

            insert_all_ingredients_of_recipe(ingredient_repo, &initial_recipe).await;

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
    }

    mod in_memory {
        use crate::domain::repositories::{
            ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
        };

        #[tokio::test]
        async fn deleting_an_existing_ingredient_works() {
            let repo = InMemoryRecipeRepository::new();
            let ingredient_repo = InMemoryIngredientRepository::new();
            super::__tests__::deleting_an_existing_ingredient_works(repo, ingredient_repo).await;
        }

        #[tokio::test]
        async fn deleting_an_ingredient_that_doesnt_appear_in_recipe_errors() {
            let repo = InMemoryRecipeRepository::new();
            let ingredient_repo = InMemoryIngredientRepository::new();
            super::__tests__::deleting_an_ingredient_that_doesnt_appear_in_recipe_errors(
                repo,
                ingredient_repo,
            )
            .await;
        }

        #[tokio::test]
        async fn deleting_an_ingredient_in_recipe_that_doesnt_exist_errors() {
            let repo = InMemoryRecipeRepository::new();
            let ingredient_repo = InMemoryIngredientRepository::new();
            super::__tests__::deleting_an_ingredient_in_recipe_that_doesnt_exist_errors(
                repo,
                ingredient_repo,
            )
            .await;
        }

        #[tokio::test]
        async fn deleting_the_last_ingredient_in_recipe_errors() {
            let repo = InMemoryRecipeRepository::new();
            let ingredient_repo = InMemoryIngredientRepository::new();
            super::__tests__::deleting_the_last_ingredient_in_recipe_errors(repo, ingredient_repo)
                .await;
        }
    }

    mod sql {
        use sqlx::PgPool;

        use crate::domain::repositories::{
            ingredients::postgres::PostgresIngredientRepository,
            recipe::postgres::PostgresRecipeRepository,
        };

        #[sqlx::test]
        async fn deleting_an_existing_ingredient_works(pool: PgPool) {
            let repo = PostgresRecipeRepository::new(pool.clone());
            let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
            super::__tests__::deleting_an_existing_ingredient_works(repo, ingredient_repo).await;
        }

        #[sqlx::test]
        async fn deleting_an_ingredient_that_doesnt_appear_in_recipe_errors(pool: PgPool) {
            let repo = PostgresRecipeRepository::new(pool.clone());
            let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
            super::__tests__::deleting_an_ingredient_that_doesnt_appear_in_recipe_errors(
                repo,
                ingredient_repo,
            )
            .await;
        }

        #[sqlx::test]
        async fn deleting_an_ingredient_in_recipe_that_doesnt_exist_errors(pool: PgPool) {
            let repo = PostgresRecipeRepository::new(pool.clone());
            let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
            super::__tests__::deleting_an_ingredient_in_recipe_that_doesnt_exist_errors(
                repo,
                ingredient_repo,
            )
            .await;
        }

        #[sqlx::test]
        async fn deleting_the_last_ingredient_in_recipe_errors(pool: PgPool) {
            let repo = PostgresRecipeRepository::new(pool.clone());
            let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
            super::__tests__::deleting_the_last_ingredient_in_recipe_errors(repo, ingredient_repo)
                .await;
        }
    }
}
