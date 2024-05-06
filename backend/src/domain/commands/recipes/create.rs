use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{
    entities::recipe::Recipe,
    repositories::{
        ingredients::{errors::IngredientRepositoryError, IngredientRepositoryService},
        recipe::{errors::RecipeRepositoryError, RecipeRepositoryService},
    },
};

#[derive(thiserror::Error, Debug)]
pub enum CreateRecipeError {
    #[error("Could not find the ingredients with the following IDs: {0:?}")]
    IngredientsNotFound(Vec<Uuid>),
    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl From<RecipeRepositoryError> for CreateRecipeError {
    fn from(value: RecipeRepositoryError) -> Self {
        match value {
            RecipeRepositoryError::UnknownError(e) => Self::Unknown(e),
            _ => unreachable!(),
        }
    }
}

impl From<IngredientRepositoryError> for CreateRecipeError {
    fn from(value: IngredientRepositoryError) -> Self {
        match value {
            IngredientRepositoryError::UnknownError(e) => Self::Unknown(e),
            IngredientRepositoryError::MultipleMissing(ids) => Self::IngredientsNotFound(ids),
            _ => unreachable!(),
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
}

#[derive(Debug)]
pub struct IngredientAmountData {
    pub ingredient_id: Uuid,
    pub amount: String,
}

pub async fn create_recipe(
    recipe_repo: RecipeRepositoryService,
    ingredient_repo: IngredientRepositoryService,
    input: &CreateRecipe<'_>,
) -> Result<Recipe, CreateRecipeError> {
    let ingredient_ids: Vec<Uuid> = input.ingredients.iter().map(|i| i.ingredient_id).collect();
    let ingredients_in_recipe = ingredient_repo
        .get_all_by_id(&ingredient_ids)
        .await
        .map_err(CreateRecipeError::from)?;

    dbg!(&ingredient_ids);
    dbg!(ingredients_in_recipe);

    todo!();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::domain::repositories::{
        ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
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
                    amount: "1 gram".to_string(),
                }],
            },
        )
        .await
        .unwrap_err();

        assert!(matches!(result, CreateRecipeError::IngredientsNotFound(_)));
    }
}
