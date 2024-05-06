use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use async_trait::async_trait;
use eyre::eyre;
use uuid::Uuid;

use crate::domain::entities::ingredient::{
    errors::ValidationError, Ingredient, IngredientChangeset,
};

use super::{errors::IngredientRepositoryError, IngredientRepository};

pub struct InMemoryIngredientRepository(pub Mutex<HashMap<Uuid, Ingredient>>);

#[async_trait]
impl IngredientRepository for InMemoryIngredientRepository {
    #[tracing::instrument(
        "[INGREDIENT REPOSITORY] [IN MEMORY] Insert a new ingredient",
        skip(self)
    )]
    async fn insert(
        &self,
        ingredient: Ingredient,
    ) -> Result<Ingredient, IngredientRepositoryError> {
        let mut lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        if lock.iter().any(|(id, _)| id == &ingredient.id) {
            tracing::error!("The ingredient with ID {} already exists.", ingredient.id);
            return Err(IngredientRepositoryError::Conflict("id".to_string()));
        };

        if lock.iter().any(|(_id, x)| x.name == ingredient.name) {
            tracing::error!(
                "The ingredient with name {} already exists.",
                ingredient.name
            );
            return Err(IngredientRepositoryError::Conflict("name".to_string()));
        };

        lock.insert(ingredient.id, ingredient.clone());

        Ok(ingredient)
    }

    #[tracing::instrument(
        "[INGREDIENT REPOSITORY] [IN MEMORY] Get ingredient with ID",
        skip(self)
    )]
    async fn get_by_id(&self, id: Uuid) -> Result<Ingredient, IngredientRepositoryError> {
        let lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        let ingredient = lock
            .values()
            .find(|x| x.id == id)
            .ok_or(IngredientRepositoryError::NotFound(id))?;

        Ok(ingredient.clone())
    }

    #[tracing::instrument("[INGREDIENT REPOSITORY] [IN MEMORY] Get all ingredients", skip(self))]
    async fn get_all(&self) -> Result<Vec<Ingredient>, IngredientRepositoryError> {
        let lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        Ok(lock.values().cloned().collect())
    }

    #[tracing::instrument("[INGREDIENT REPOSITORY] [IN MEMORY] Update ingredient", skip(self))]
    async fn update(
        &self,
        id: Uuid,
        changeset: IngredientChangeset,
    ) -> Result<Ingredient, IngredientRepositoryError> {
        let mut lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        let ingredient = lock
            .get_mut(&id)
            .ok_or(IngredientRepositoryError::NotFound(id))?;

        let name: Option<String> = changeset.name.map(|n| n.to_string());
        let description: Option<String> = changeset.description.map(|n| n.to_string());
        let diet_friendly: Option<Vec<String>> = changeset.diet_friendly.map(|df| df.into());

        if name.is_none() && description.is_none() && diet_friendly.is_none() {
            return Err(IngredientRepositoryError::ValidationError(
                ValidationError::EmptyField(vec!["name", "description", "diet_friendly"]),
            ));
        };

        if let Some(new_name) = name {
            ingredient.name = new_name.try_into()?;
        }

        if let Some(new_description) = description {
            ingredient.description = new_description.try_into()?;
        }

        if let Some(new_diets) = diet_friendly {
            ingredient.diet_friendly = new_diets.into();
        }

        Ok(ingredient.clone())
    }

    #[tracing::instrument("[INGREDIENT REPOSITORY] [IN MEMORY] Delete an ingredient", skip(self))]
    async fn delete(&self, id: Uuid) -> Result<(), IngredientRepositoryError> {
        let ingredient = self.get_by_id(id).await?;
        let mut lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;
        lock.remove(&ingredient.id);

        Ok(())
    }

    async fn get_all_by_id(
        &self,
        ids: &Vec<Uuid>,
    ) -> Result<Vec<Ingredient>, IngredientRepositoryError> {
        let lock = self.0.lock().map_err(|_| {
            eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked")
        })?;

        let mut missing_ids: HashSet<Uuid> = HashSet::from_iter(ids.iter().cloned());

        let collect = lock
            .clone()
            .into_iter()
            .filter_map(|(id, ingredient)| {
                dbg!(&id);
                dbg!(&ids);
                if ids.contains(&id) {
                    missing_ids.remove(&id);
                    Some(ingredient)
                } else {
                    None
                }
            })
            .collect();

        if !missing_ids.is_empty() {
            Err(IngredientRepositoryError::MultipleMissing(
                missing_ids.iter().cloned().collect(),
            ))
        } else {
            Ok(collect)
        }
    }
}

impl InMemoryIngredientRepository {
    pub fn new() -> Self {
        HashMap::new().into()
    }
}

impl Default for InMemoryIngredientRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<Uuid, Ingredient>> for InMemoryIngredientRepository {
    fn from(value: HashMap<Uuid, Ingredient>) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
mod tests;
