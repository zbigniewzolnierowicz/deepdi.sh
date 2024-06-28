pub mod errors;
pub mod in_memory;
pub mod postgres;

#[cfg(test)]
pub mod __test__;

use crate::domain::entities::recipe::{Recipe, RecipeChangeset};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use self::errors::{DeleteRecipeError, GetRecipeByIdError, InsertRecipeError, UpdateRecipeError};

#[async_trait]
pub trait RecipeRepository: Send + Sync + 'static {
    // TODO: Include user information
    async fn insert(&self, input: Recipe) -> Result<Recipe, InsertRecipeError>;

    async fn get_by_id(&self, id: &Uuid) -> Result<Recipe, GetRecipeByIdError>;

    async fn delete(&self, id: &Uuid) -> Result<(), DeleteRecipeError>;

    async fn update(
        &self,
        id: &Uuid,
        changeset: RecipeChangeset,
    ) -> Result<Recipe, UpdateRecipeError>;
}

pub type RecipeRepositoryService = Arc<Box<dyn RecipeRepository>>;
