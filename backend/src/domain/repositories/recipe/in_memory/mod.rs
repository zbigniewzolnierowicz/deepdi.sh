use async_trait::async_trait;
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

use crate::domain::{
    entities::{ingredient::Ingredient, recipe::{Recipe, RecipeChangeset}},
    repositories::recipe::errors::InsertRecipeError,
};

use super::{
    errors::{DeleteRecipeError, GetRecipeByIdError, UpdateRecipeError},
    RecipeRepository,
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
        id: &Uuid,
        changeset: RecipeChangeset,
    ) -> Result<Recipe, UpdateRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe = lock
            .get_mut(id)
            .ok_or(UpdateRecipeError::Get(GetRecipeByIdError::NotFound(*id)))?;

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

        Ok(recipe.clone())
    }

    async fn add_ingredient(&self, id: &Uuid, ingredient: Ingredient) -> Result<Recipe, UpdateRecipeError> {
        todo!()
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
}

impl From<HashMap<uuid::Uuid, Recipe>> for InMemoryRecipeRepository {
    fn from(value: HashMap<uuid::Uuid, Recipe>) -> Self {
        Self(Mutex::new(value))
    }
}

#[cfg(test)]
mod tests;
