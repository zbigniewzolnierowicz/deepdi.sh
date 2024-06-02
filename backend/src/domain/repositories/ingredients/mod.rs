pub mod errors;
pub mod postgres;

pub mod in_memory;

#[cfg(test)]
pub use in_memory::InMemoryIngredientRepository;

use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::ingredient::{Ingredient, IngredientChangeset};

use self::errors::{
    DeleteIngredientError, GetAllIngredientsError, GetIngredientByIdError, InsertIngredientError,
    UpdateIngredientError,
};

#[async_trait]
pub trait IngredientRepository: Send + Sync + 'static {
    async fn insert(&self, ingredient: Ingredient) -> Result<Ingredient, InsertIngredientError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Ingredient, GetIngredientByIdError>;
    async fn get_all_by_id(&self, ids: &[Uuid]) -> Result<Vec<Ingredient>, GetAllIngredientsError>;
    async fn get_all(&self) -> Result<Vec<Ingredient>, GetAllIngredientsError>;
    async fn update(
        &self,
        id: Uuid,
        changeset: IngredientChangeset,
    ) -> Result<Ingredient, UpdateIngredientError>;
    async fn delete(&self, id: Uuid) -> Result<(), DeleteIngredientError>;
}

pub type IngredientRepositoryService = Arc<Box<dyn IngredientRepository>>;
