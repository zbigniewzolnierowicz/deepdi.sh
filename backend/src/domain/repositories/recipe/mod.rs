pub mod errors;
pub mod in_memory;
pub mod postgres;

use crate::domain::entities::recipe::Recipe;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use self::errors::{DeleteRecipeError, GetRecipeByIdError, InsertRecipeError};

#[async_trait]
pub trait RecipeRepository: Send + Sync + 'static {
    // TODO: Include user information
    async fn insert(&self, input: Recipe) -> Result<Recipe, InsertRecipeError>;

    async fn get_by_id(&self, id: &Uuid) -> Result<Recipe, GetRecipeByIdError>;

    async fn delete(&self, id: &Uuid) -> Result<(), DeleteRecipeError>;
}

pub type RecipeRepositoryService = Arc<Box<dyn RecipeRepository>>;
