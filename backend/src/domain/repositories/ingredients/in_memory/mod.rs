use std::{
    collections::{BTreeMap, HashSet},
    sync::Mutex,
};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::ingredient::{
    errors::ValidationError, Ingredient, IngredientChangeset,
};

use super::{
    errors::{
        DeleteIngredientError, GetAllIngredientsError, GetIngredientByIdError,
        InsertIngredientError, UpdateIngredientError,
    },
    IngredientRepository,
};

pub struct InMemoryIngredientRepository(pub Mutex<BTreeMap<Uuid, Ingredient>>);

#[async_trait]
impl IngredientRepository for InMemoryIngredientRepository {
    #[tracing::instrument(
        "[INGREDIENT REPOSITORY] [IN MEMORY] Insert a new ingredient",
        skip(self)
    )]
    async fn insert(&self, ingredient: Ingredient) -> Result<Ingredient, InsertIngredientError> {
        let mut lock = self.0.lock()?;

        if lock.iter().any(|(id, _)| id == &ingredient.id) {
            tracing::error!("The ingredient with ID {} already exists.", ingredient.id);
            return Err(InsertIngredientError::Conflict("id".to_string()));
        };

        if lock.iter().any(|(_id, x)| x.name == ingredient.name) {
            tracing::error!(
                "The ingredient with name {} already exists.",
                ingredient.name
            );
            return Err(InsertIngredientError::Conflict("name".to_string()));
        };

        lock.insert(ingredient.id, ingredient.clone());

        Ok(ingredient)
    }

    #[tracing::instrument(
        "[INGREDIENT REPOSITORY] [IN MEMORY] Get ingredient with ID",
        skip(self)
    )]
    async fn get_by_id(&self, id: &Uuid) -> Result<Ingredient, GetIngredientByIdError> {
        let lock = self.0.lock()?;

        let ingredient = lock
            .values()
            .find(|x| x.id == *id)
            .ok_or(GetIngredientByIdError::NotFound(*id))?;

        Ok(ingredient.clone())
    }

    #[tracing::instrument("[INGREDIENT REPOSITORY] [IN MEMORY] Get all ingredients", skip(self))]
    async fn get_all(&self) -> Result<Vec<Ingredient>, GetAllIngredientsError> {
        let lock = self.0.lock()?;

        Ok(lock.values().cloned().collect())
    }

    #[tracing::instrument("[INGREDIENT REPOSITORY] [IN MEMORY] Update ingredient", skip(self))]
    async fn update(
        &self,
        id: Uuid,
        changeset: IngredientChangeset,
    ) -> Result<Ingredient, UpdateIngredientError> {
        let mut lock = self.0.lock()?;

        let ingredient = lock
            .get_mut(&id)
            .ok_or(GetIngredientByIdError::NotFound(id))?;

        let name: Option<String> = changeset.name.map(|n| n.to_string());
        let description: Option<String> = changeset.description.map(|n| n.to_string());
        let diet_friendly: Option<Vec<String>> = changeset.diet_friendly.map(|df| df.into());

        if name.is_none() && description.is_none() && diet_friendly.is_none() {
            return Err(UpdateIngredientError::ValidationError(
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
    async fn delete(&self, id: Uuid) -> Result<(), DeleteIngredientError> {
        let ingredient = self.get_by_id(&id).await?;
        let mut lock = self.0.lock()?;
        lock.remove(&ingredient.id);

        Ok(())
    }

    async fn get_all_by_id(&self, ids: &[Uuid]) -> Result<Vec<Ingredient>, GetAllIngredientsError> {
        let lock = self.0.lock()?;

        let mut missing_ids: HashSet<Uuid> = HashSet::from_iter(ids.iter().cloned());

        let collect = lock
            .clone()
            .into_iter()
            .filter_map(|(id, ingredient)| {
                if ids.contains(&id) {
                    missing_ids.remove(&id);
                    Some(ingredient)
                } else {
                    None
                }
            })
            .collect();

        if !missing_ids.is_empty() {
            Err(GetAllIngredientsError::MultipleIngredientsMissing(
                missing_ids.iter().cloned().collect(),
            ))
        } else {
            Ok(collect)
        }
    }
}

impl InMemoryIngredientRepository {
    pub fn new() -> Self {
        BTreeMap::new().into()
    }
}

impl Default for InMemoryIngredientRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BTreeMap<Uuid, Ingredient>> for InMemoryIngredientRepository {
    fn from(value: BTreeMap<Uuid, Ingredient>) -> Self {
        Self(value.into())
    }
}

#[cfg(test)]
mod tests;
