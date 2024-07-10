use async_trait::async_trait;
use eyre::eyre;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::domain::{
    entities::recipe::{IngredientUnit, IngredientWithAmount, Recipe, RecipeChangeset},
    repositories::recipe::errors::InsertRecipeError,
};

use super::{
    errors::{
        AddIngredientIntoRecipeError, DeleteIngredientFromRecipeError, DeleteRecipeError,
        GetRecipeByIdError, UpdateIngredientInRecipeError, UpdateRecipeError,
    },
    RecipeRepository, RecipeRepositoryService,
};

pub struct InMemoryRecipeRepository(pub Mutex<HashMap<uuid::Uuid, Recipe>>);

#[async_trait]
impl RecipeRepository for InMemoryRecipeRepository {
    async fn insert(&self, input: Recipe) -> Result<Recipe, InsertRecipeError> {
        let mut lock = self.0.lock()?;

        if lock.iter().any(|(id, _)| id == &input.id) {
            tracing::error!("The recipe with ID {} already exists.", input.id);
            return Err(InsertRecipeError::Conflict("recipe id".to_string()));
        };

        lock.insert(input.id, input.clone());

        Ok(input)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Recipe, GetRecipeByIdError> {
        let lock = self.0.lock()?;

        let result = lock
            .get(id)
            .cloned()
            .ok_or_else(|| GetRecipeByIdError::NotFound(*id))?;

        Ok(result)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DeleteRecipeError> {
        let mut lock = self.0.lock()?;

        lock.remove(id).ok_or(DeleteRecipeError::NotFound(*id))?;

        Ok(())
    }

    async fn update(
        &self,
        recipe: &Recipe,
        changeset: RecipeChangeset,
    ) -> Result<(), UpdateRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe = lock
            .get_mut(&recipe.id)
            .ok_or(UpdateRecipeError::UnknownError(eyre!("The recipe could not be found somehow")))?;

        if let Some(v) = changeset.name {
            recipe.name = v;
        };

        if let Some(v) = changeset.time {
            recipe.time = v;
        };

        if let Some(v) = changeset.steps {
            recipe.steps = v;
        };

        if let Some(v) = changeset.servings {
            recipe.servings = v;
        };

        if let Some(v) = changeset.description {
            recipe.description = v;
        };

        Ok(())
    }

    async fn add_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: IngredientWithAmount,
    ) -> Result<(), AddIngredientIntoRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe = lock
            .get_mut(&recipe.id)
            .ok_or(AddIngredientIntoRecipeError::UnknownError(eyre!(
                "Recipe is not in the repo"
            )))?;

        recipe.ingredients.push(ingredient);

        Ok(())
    }

    async fn delete_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
    ) -> Result<(), DeleteIngredientFromRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe =
            lock.get_mut(&recipe.id)
                .ok_or(DeleteIngredientFromRecipeError::UnknownError(eyre!(
                    "Recipe is not in the repo somehow"
                )))?;

        let new_ingredients: Vec<_> = recipe
            .ingredients
            .iter()
            .cloned()
            .filter(|ingam| ingam.ingredient.id != ingredient.ingredient.id)
            .collect();

        recipe.ingredients = new_ingredients
            .try_into()
            .map_err(|e| DeleteIngredientFromRecipeError::ValidationError(e))?;

        Ok(())
    }

    async fn update_ingredient_amount(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
        new_amount: &IngredientUnit,
    ) -> Result<(), UpdateIngredientInRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe =
            lock.get_mut(&recipe.id)
                .ok_or(UpdateIngredientInRecipeError::UnknownError(eyre!(
                    "Recipe is not in the repo somehow"
                )))?;

        let ingredient = recipe
            .ingredients
            .iter_mut()
            .find(|i| i.ingredient.id == ingredient.ingredient.id)
            .ok_or(UpdateIngredientInRecipeError::UnknownError(eyre!(
                "Ingredient somehow is not in the recipe, but the command made sure there was."
            )))?;
        
        ingredient.amount = new_amount.clone();

        Ok(())
    }
}

impl Default for InMemoryRecipeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryRecipeRepository {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    pub fn service(self) -> RecipeRepositoryService {
        Arc::new(Box::new(self))
    }
}

impl From<HashMap<uuid::Uuid, Recipe>> for InMemoryRecipeRepository {
    fn from(value: HashMap<uuid::Uuid, Recipe>) -> Self {
        Self(Mutex::new(value))
    }
}

#[cfg(test)]
mod tests;
