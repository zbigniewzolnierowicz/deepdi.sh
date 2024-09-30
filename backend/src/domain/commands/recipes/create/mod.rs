use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use uuid::Uuid;

use crate::domain::entities::recipe::IngredientAmountData;
use crate::domain::entities::recipe::{
    errors::ValidationError, IngredientWithAmount, Recipe, ServingsType,
};
use crate::domain::repositories::recipe::errors::GetRecipeByIdError;
use crate::domain::repositories::{
    ingredients::{errors::GetAllIngredientsError, IngredientRepositoryService},
    recipe::{errors::InsertRecipeError, RecipeRepositoryService},
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

impl From<InsertRecipeError> for CreateRecipeError {
    fn from(value: InsertRecipeError) -> Self {
        match value {
            InsertRecipeError::ValidationError(e) => Self::Validation(e),
            e => Self::Unknown(e.into()),
        }
    }
}

impl From<GetAllIngredientsError> for CreateRecipeError {
    fn from(value: GetAllIngredientsError) -> Self {
        match value {
            GetAllIngredientsError::MultipleIngredientsMissing(ids) => {
                Self::IngredientsNotFound(ids)
            }
            e => Self::Unknown(e.into()),
        }
    }
}

impl From<GetRecipeByIdError> for CreateRecipeError {
    fn from(value: GetRecipeByIdError) -> Self {
        Self::Unknown(value.into())
    }
}

#[derive(Debug)]
pub struct CreateRecipe {
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub time: BTreeMap<String, std::time::Duration>,
    pub ingredients: Vec<IngredientAmountData>,
    pub servings: ServingsType,
}

impl From<Recipe> for CreateRecipe {
    fn from(value: Recipe) -> Self {
        Self {
            name: value.name,
            description: value.description,
            servings: value.servings,
            steps: value.steps.as_ref().to_vec(),
            time: value.time,
            ingredients: value.ingredients.into(),
        }
    }
}

pub async fn create_recipe(
    recipe_repo: RecipeRepositoryService,
    ingredient_repo: IngredientRepositoryService,
    input: &CreateRecipe,
) -> Result<Recipe, CreateRecipeError> {
    let ingredient_ids: Vec<Uuid> = input.ingredients.iter().map(|i| i.ingredient_id).collect();

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

    let id = Uuid::now_v7();
    let created_at: DateTime<Utc> = Utc::now();
    let updated_at: DateTime<Utc> = Utc::now();

    // FIXME: make the database take care of this
    recipe_repo
        .insert(Recipe {
            id,
            name: input.name.to_string(),
            description: input.description.to_string(),
            steps: input.steps.clone().try_into()?,
            ingredients: ingredients_in_recipe.try_into()?,
            time: input.time.clone(),
            servings: input.servings.clone(),
            created_at,
            updated_at,
        })
        .await?;

    let recipe = recipe_repo.get_by_id(&id).await?;

    Ok(recipe)
}

#[cfg(test)]
mod tests;
