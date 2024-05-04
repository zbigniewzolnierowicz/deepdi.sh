pub mod errors;
pub mod in_memory;

use crate::domain::entities::recipe::Recipe;
use async_trait::async_trait;
use std::sync::Arc;

use self::errors::RecipeRepositoryError;

#[async_trait]
pub trait RecipeRepository: Send + Sync + 'static {
    // TODO: Include user information
    async fn insert(&self, input: Recipe) -> Result<Recipe, RecipeRepositoryError>;
}

pub type RecipeRepositoryService = Arc<Box<dyn RecipeRepository>>;
