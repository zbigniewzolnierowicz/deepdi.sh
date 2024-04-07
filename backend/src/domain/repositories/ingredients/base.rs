use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::ingredient::{Ingredient, IngredientChangeset};

use super::errors::IngredientRepositoryError;

#[async_trait]
pub trait IngredientRepository: Send + Sync + 'static {
    async fn insert(&self, ingredient: Ingredient)
        -> Result<Ingredient, IngredientRepositoryError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Ingredient, IngredientRepositoryError>;
    async fn get_all(&self) -> Result<Vec<Ingredient>, IngredientRepositoryError>;
    async fn update(
        &mut self,
        id: Uuid,
        changeset: IngredientChangeset,
    ) -> Result<Ingredient, IngredientRepositoryError>;
}

pub type IngredientRepositoryService = Arc<Box<dyn IngredientRepository>>;
