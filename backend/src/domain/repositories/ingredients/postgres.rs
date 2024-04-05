use async_trait::async_trait;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::ingredient::{Ingredient, IngredientModel};

use super::{base::IngredientRepository, errors::IngredientRepositoryError};

pub struct PostgresIngredientRepository(pub PgPool);

#[async_trait]
impl IngredientRepository for PostgresIngredientRepository {
    async fn insert(
        &self,
        ingredient: Ingredient,
    ) -> Result<Ingredient, IngredientRepositoryError> {
        let diet_friendly: Vec<String> = ingredient
            .clone()
            .diet_friendly
            .0
            .into_iter()
            .map(|d| d.to_string())
            .collect();

        let ingredient = sqlx::query_as!(
            IngredientModel,
            r#"
                INSERT INTO ingredients (id, name, description, diet_friendly)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, description, diet_friendly
            "#,
            ingredient.id,
            &ingredient.name,
            &ingredient.description,
            &diet_friendly
        )
        .fetch_one(&self.0)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::Database(dberr) => {
                if dberr.is_unique_violation() {
                    return IngredientRepositoryError::Conflict;
                };
                IngredientRepositoryError::UnknownError(dberr.into())
            }
            err => IngredientRepositoryError::UnknownError(err.into()),
        })?;

        Ok(ingredient.try_into()?)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Ingredient, IngredientRepositoryError> {
        let ingredient = sqlx::query_as!(
            IngredientModel,
            r#"
            SELECT id, name, description, diet_friendly
            FROM ingredients
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.0)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => IngredientRepositoryError::NotFound(id),
            e => IngredientRepositoryError::UnknownError(e.into()),
        })?;

        Ok(ingredient.try_into()?)
    }

    async fn get_all(&self) -> Result<Vec<Ingredient>, IngredientRepositoryError> {
        let ingredients = sqlx::query_as!(
            IngredientModel,
            r#"
            SELECT id, name, description, diet_friendly
            FROM ingredients;
            "#
        )
        .fetch_all(&self.0)
        .await
        .map_err(|e| IngredientRepositoryError::UnknownError(e.into()))?
        .par_iter()
        .filter_map(|i| i.try_into().ok())
        .collect();

        Ok(ingredients)
    }
}

impl PostgresIngredientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}
