use std::sync::Mutex;

use async_trait::async_trait;
use eyre::eyre;
use uuid::Uuid;

use crate::domain::entities::ingredient::Ingredient;

#[async_trait]
pub trait IngredientRepository: Send + Sync {
    async fn insert(&self, ingredient: Ingredient)
        -> Result<Ingredient, IngredientRepositoryError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Ingredient, IngredientRepositoryError>;
}

pub struct InMemoryIngredientRepository(pub Mutex<Vec<Ingredient>>);

#[derive(thiserror::Error, Debug)]
pub enum IngredientRepositoryError {
    #[error("The ingredient with ID of {0} was not found")]
    NotFound(Uuid),
    #[error("There was a conflict when inserting")]
    Conflict,
    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

#[async_trait]
impl IngredientRepository for InMemoryIngredientRepository {
    async fn insert(
        &self,
        ingredient: Ingredient,
    ) -> Result<Ingredient, IngredientRepositoryError> {
        let mut lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        if lock
            .iter()
            .any(|x| x.id == ingredient.id || x.name == ingredient.name)
        {
            return Err(IngredientRepositoryError::Conflict);
        };

        lock.push(ingredient.clone());

        Ok(ingredient)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Ingredient, IngredientRepositoryError> {
        let lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        let ingredient = lock
            .iter()
            .find(|x| x.id == id)
            .ok_or(IngredientRepositoryError::NotFound(id))?;

        Ok(ingredient.clone())
    }
}

impl InMemoryIngredientRepository {
    pub fn new() -> Self {
        Self(Mutex::new(vec![]))
    }
}
