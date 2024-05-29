use std::collections::HashMap;

use axum::response::IntoResponse;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    entities::recipe::{
        errors::ValidationError, IngredientUnit, IngredientWithAmount, Recipe, ServingsType,
    },
    repositories::{
        ingredients::{errors::IngredientRepositoryError, IngredientRepositoryService},
        recipe::{errors::InsertRecipeError, RecipeRepositoryService},
    },
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum CreateRecipeError {
    #[error("Could not find the ingredients with the following IDs: {0:?}")]
    IngredientsNotFound(Vec<Uuid>),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl IntoResponse for CreateRecipeError {
    fn into_response(self) -> axum::response::Response {
        let error_type: &str = self.as_ref();
        let status = match self {
            Self::IngredientsNotFound(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (
            status,
            axum::Json(common::error::ErrorMessage::new(
                error_type,
                self.to_string(),
            )),
        )
            .into_response()
    }
}

impl From<InsertRecipeError> for CreateRecipeError {
    fn from(value: InsertRecipeError) -> Self {
        match value {
            InsertRecipeError::ValidationError(e) => Self::Validation(e),
            e => Self::Unknown(e.into()),
        }
    }
}

impl From<IngredientRepositoryError> for CreateRecipeError {
    fn from(value: IngredientRepositoryError) -> Self {
        match value {
            IngredientRepositoryError::MultipleIngredientsMissing(ids) => {
                Self::IngredientsNotFound(ids)
            }
            e => Self::Unknown(e.into()),
        }
    }
}

#[derive(Debug)]
pub struct CreateRecipe<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub steps: Vec<String>,
    pub time: HashMap<String, std::time::Duration>,
    pub ingredients: Vec<IngredientAmountData>,
    pub servings: ServingsType,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IngredientAmountData {
    pub ingredient_id: Uuid,
    pub amount: IngredientUnit,
    pub optional: bool,
    pub notes: Option<String>,
}

pub async fn create_recipe(
    recipe_repo: RecipeRepositoryService,
    ingredient_repo: IngredientRepositoryService,
    input: &CreateRecipe<'_>,
) -> Result<Recipe, CreateRecipeError> {
    let ingredient_ids: Vec<Uuid> = input.ingredients.iter().map(|i| i.ingredient_id).collect();

    // FIXME: There's probably a better way to do this
    let ingredients_in_recipe: Vec<_> = ingredient_repo
        .get_all_by_id(&ingredient_ids)
        .await
        .map_err(CreateRecipeError::from)?
        .into_par_iter()
        .zip(&input.ingredients)
        .map(
            |(
                ingredient,
                IngredientAmountData {
                    amount,
                    optional,
                    notes,
                    ..
                },
            )| {
                IngredientWithAmount {
                    ingredient,
                    amount: amount.clone(),
                    notes: notes.clone(),
                    optional: *optional,
                }
            },
        )
        .collect();

    let recipe = recipe_repo
        .insert(Recipe {
            id: Uuid::now_v7(),
            name: input.name.to_string(),
            description: input.description.to_string(),
            steps: input.steps.clone(),
            ingredients: ingredients_in_recipe,
            time: input.time.clone(),
            servings: input.servings.clone(),
        })
        .await?;

    Ok(recipe)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sqlx::PgPool;

    use super::*;
    use crate::domain::{
        entities::ingredient::{types::DietFriendly, Ingredient, IngredientModel},
        repositories::{
            ingredients::{postgres::PostgresIngredientRepository, InMemoryIngredientRepository},
            recipe::{in_memory::InMemoryRecipeRepository, postgres::PostgresRecipeRepository},
        },
    };

    #[tokio::test]
    async fn create_recipe_without_proper_ingredients_errors() {
        let ingredient_repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));
        let recipe_repo: RecipeRepositoryService =
            Arc::new(Box::new(InMemoryRecipeRepository::new()));

        let result = create_recipe(
            recipe_repo,
            ingredient_repo,
            &CreateRecipe {
                name: "Recipe test",
                description: "This is a test for the recipe",
                time: HashMap::from([(
                    "Prep time".to_string(),
                    std::time::Duration::from_secs(60),
                )]),
                steps: vec!["Try screaming at the food".to_string()],
                ingredients: vec![IngredientAmountData {
                    ingredient_id: Uuid::from_u128(0),
                    amount: IngredientUnit::Grams(1.0),
                    ..Default::default()
                }],
                servings: ServingsType::FromTo(1, 2),
            },
        )
        .await
        .unwrap_err();

        assert!(matches!(result, CreateRecipeError::IngredientsNotFound(_)));
    }

    #[tokio::test]
    async fn create_recipe_with_proper_ingredients() {
        let ingredients: HashMap<Uuid, Ingredient> = HashMap::from([(
            Uuid::from_u128(0),
            IngredientModel {
                id: Uuid::from_u128(0),
                name: "Ingredient Zero".to_string(),
                description: "Description of ingredient zero".to_string(),
                diet_friendly: vec![
                    DietFriendly::Vegan.to_string(),
                    DietFriendly::Vegetarian.to_string(),
                    DietFriendly::GlutenFree.to_string(),
                ],
            }
            .try_into()
            .unwrap(),
        )]);

        let internal_ingredient_repo: InMemoryIngredientRepository = ingredients.into();
        let ingredient_repo: IngredientRepositoryService =
            Arc::new(Box::new(internal_ingredient_repo));

        let recipe_repo: RecipeRepositoryService =
            Arc::new(Box::new(InMemoryRecipeRepository::new()));

        let result = create_recipe(
            recipe_repo,
            ingredient_repo,
            &CreateRecipe {
                name: "Recipe test",
                description: "This is a test for the recipe",
                time: HashMap::from([(
                    "Prep time".to_string(),
                    std::time::Duration::from_secs(60),
                )]),
                steps: vec!["Try screaming at the food".to_string()],
                ingredients: vec![IngredientAmountData {
                    ingredient_id: Uuid::from_u128(0),
                    amount: IngredientUnit::Grams(1.0),
                    ..Default::default()
                }],
                servings: ServingsType::FromTo(1, 2),
            },
        )
        .await
        .unwrap();

        assert_eq!(Uuid::get_version(&result.id), Some(uuid::Version::SortRand));
        assert_eq!(&result.name, "Recipe test");
        assert_eq!(
            result.ingredients,
            vec![IngredientWithAmount {
                ingredient: IngredientModel {
                    id: Uuid::from_u128(0),
                    name: "Ingredient Zero".to_string(),
                    description: "Description of ingredient zero".to_string(),
                    diet_friendly: vec![
                        DietFriendly::Vegan.to_string(),
                        DietFriendly::Vegetarian.to_string(),
                        DietFriendly::GlutenFree.to_string(),
                    ],
                }
                .try_into()
                .unwrap(),
                amount: IngredientUnit::Grams(1.0),
                notes: None,
                optional: false,
            }]
        )
    }
    #[sqlx::test]
    async fn create_recipe_without_proper_ingredients_postgres(pool: PgPool) {
        let ingredient_repo: IngredientRepositoryService =
            Arc::new(Box::new(PostgresIngredientRepository::new(pool.clone())));

        let recipe_repo: RecipeRepositoryService =
            Arc::new(Box::new(PostgresRecipeRepository::new(pool)));

        let result = create_recipe(
            recipe_repo,
            ingredient_repo,
            &CreateRecipe {
                name: "Recipe test",
                description: "This is a test for the recipe",
                time: HashMap::from([(
                    "Prep time".to_string(),
                    std::time::Duration::from_secs(60),
                )]),
                steps: vec!["Try screaming at the food".to_string()],
                ingredients: vec![IngredientAmountData {
                    ingredient_id: Uuid::from_u128(0),
                    amount: IngredientUnit::Grams(1.0),
                    ..Default::default()
                }],
                servings: ServingsType::FromTo(1, 2),
            },
        )
        .await
        .unwrap_err();

        assert!(matches!(result, CreateRecipeError::IngredientsNotFound(_)));
    }

    #[sqlx::test]
    async fn create_recipe_with_proper_ingredients_postgres(pool: PgPool) {
        let ingredient_repo: IngredientRepositoryService =
            Arc::new(Box::new(PostgresIngredientRepository::new(pool.clone())));

        ingredient_repo
            .insert(
                IngredientModel {
                    id: Uuid::from_u128(0),
                    name: "Ingredient Zero".to_string(),
                    description: "Description of ingredient zero".to_string(),
                    diet_friendly: vec![
                        DietFriendly::Vegan.to_string(),
                        DietFriendly::Vegetarian.to_string(),
                        DietFriendly::GlutenFree.to_string(),
                    ],
                }
                .try_into()
                .unwrap(),
            )
            .await
            .unwrap();

        let recipe_repo: RecipeRepositoryService =
            Arc::new(Box::new(PostgresRecipeRepository::new(pool)));

        let result = create_recipe(
            recipe_repo,
            ingredient_repo,
            &CreateRecipe {
                name: "Recipe test",
                description: "This is a test for the recipe",
                time: HashMap::from([(
                    "Prep time".to_string(),
                    std::time::Duration::from_secs(60),
                )]),
                steps: vec!["Try screaming at the food".to_string()],
                ingredients: vec![IngredientAmountData {
                    ingredient_id: Uuid::from_u128(0),
                    amount: IngredientUnit::Grams(1.0),
                    ..Default::default()
                }],
                servings: ServingsType::FromTo(1, 2),
            },
        )
        .await
        .unwrap();

        assert_eq!(Uuid::get_version(&result.id), Some(uuid::Version::SortRand));
    }
}
