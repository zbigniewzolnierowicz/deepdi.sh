use async_trait::async_trait;
use eyre::eyre;
use std::{collections::HashMap, sync::Mutex};

use crate::domain::entities::recipe::Recipe;

use super::{errors::RecipeRepositoryError, RecipeRepository};

pub struct InMemoryRecipeRepository(pub Mutex<HashMap<uuid::Uuid, Recipe>>);

#[async_trait]
impl RecipeRepository for InMemoryRecipeRepository {
    async fn insert(&self, input: Recipe) -> Result<Recipe, RecipeRepositoryError> {
        let mut lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        if lock.iter().any(|(id, _)| id == &input.id) {
            tracing::error!("The recipe with ID {} already exists.", input.id);
            return Err(RecipeRepositoryError::Conflict("id".to_string()));
        };

        lock.insert(input.id, input.clone());

        Ok(input)
    }
}

impl InMemoryRecipeRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        InMemoryRecipeRepository(Mutex::new(HashMap::new()))
    }
}

#[cfg(test)]
mod tests;
