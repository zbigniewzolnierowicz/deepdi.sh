use crate::domain::entities::ingredient::{
    errors::ValidationError, Ingredient, IngredientChangeset, IngredientModel,
};
use async_trait::async_trait;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use sqlx::PgPool;
use uuid::Uuid;

use super::{base::IngredientRepository, errors::IngredientRepositoryError};

pub struct PostgresIngredientRepository(pub PgPool, Regex);

#[async_trait]
impl IngredientRepository for PostgresIngredientRepository {
    #[tracing::instrument(
        "[INGREDIENT REPOSITORY] [POSTGRES] Insert a new ingredient",
        skip(self)
    )]
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
            sqlx::error::Error::Database(dberror) if dberror.is_unique_violation() => {
                let constraint = dberror.constraint().unwrap_or_default().to_string();

                if let Some(captures) = self.1.captures(&constraint) {
                    let field = captures.name("field");
                    let id = captures.name("pkey");

                    if let Some(field) = field {
                        IngredientRepositoryError::Conflict(field.as_str().to_string())
                    } else if id.is_some() {
                        IngredientRepositoryError::Conflict("id".to_string())
                    } else {
                        IngredientRepositoryError::Conflict(constraint)
                    }
                } else {
                    IngredientRepositoryError::Conflict(constraint)
                }
            }
            _ => IngredientRepositoryError::UnknownError(e.into()),
        })?;

        Ok(ingredient.try_into()?)
    }

    #[tracing::instrument(
        "[INGREDIENT REPOSITORY] [POSTGRES] Get ingredient with ID",
        skip(self)
    )]
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
            sqlx::error::Error::RowNotFound => IngredientRepositoryError::NotFound(id),
            _ => IngredientRepositoryError::UnknownError(e.into()),
        })?;

        Ok(ingredient.try_into()?)
    }

    #[tracing::instrument("[INGREDIENT REPOSITORY] [POSTGRES] Get all ingredients", skip(self))]
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

    #[tracing::instrument("[INGREDIENT REPOSITORY] [POSTGRES] Update ingredient", skip(self))]
    async fn update(
        &self,
        id: Uuid,
        changeset: IngredientChangeset,
    ) -> Result<Ingredient, IngredientRepositoryError> {
        let mut ingredient_to_update = sqlx::query_as!(
            IngredientModel,
            r#"
            SELECT id, name, description, diet_friendly
            FROM ingredients
            WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.0)
        .await
        .map_err(|e| IngredientRepositoryError::UnknownError(e.into()))?
        .ok_or_else(|| IngredientRepositoryError::NotFound(id))?;

        let name: Option<String> = changeset.name.map(|n| n.to_string());
        let description: Option<String> = changeset.description.map(|n| n.to_string());
        let diet_friendly: Option<Vec<String>> = changeset.diet_friendly.map(|df| df.into());

        if name.is_none() && description.is_none() && diet_friendly.is_none() {
            return Err(IngredientRepositoryError::ValidationError(
                ValidationError::EmptyField(vec!["name", "description", "diet_friendly"]),
            ));
        };

        let tx = self
            .0
            .begin()
            .await
            .map_err(|e| IngredientRepositoryError::UnknownError(e.into()))?;

        if let Some(name) = name {
            if name != ingredient_to_update.name {
                ingredient_to_update = sqlx::query_as!(
                    IngredientModel,
                    r#"
                    UPDATE ingredients
                    SET
                    name = $2
                    WHERE id = $1
                    RETURNING id, name, description, diet_friendly
                "#,
                    id,
                    name,
                )
                .fetch_one(&self.0)
                .await
                .map_err(|e| IngredientRepositoryError::UnknownError(e.into()))?;
            };
        };

        if let Some(description) = description {
            if description != ingredient_to_update.description {
                ingredient_to_update = sqlx::query_as!(
                    IngredientModel,
                    r#"
                    UPDATE ingredients
                    SET
                    description = $2
                    WHERE id = $1
                    RETURNING id, name, description, diet_friendly
                    "#,
                    id,
                    description,
                )
                .fetch_one(&self.0)
                .await
                .map_err(|e| IngredientRepositoryError::UnknownError(e.into()))?;
            }
        };

        if let Some(diet_friendly) = diet_friendly {
            if diet_friendly != ingredient_to_update.diet_friendly {
                ingredient_to_update = sqlx::query_as!(
                    IngredientModel,
                    r#"
                    UPDATE ingredients
                    SET
                    diet_friendly = $2
                    WHERE id = $1
                    RETURNING id, name, description, diet_friendly
                    "#,
                    id,
                    &diet_friendly
                )
                .fetch_one(&self.0)
                .await
                .map_err(|e| IngredientRepositoryError::UnknownError(e.into()))?;
            }
        };

        tx.commit()
            .await
            .map_err(|e| IngredientRepositoryError::UnknownError(e.into()))?;

        Ok(ingredient_to_update.try_into()?)
    }
}

impl PostgresIngredientRepository {
    pub fn new(pool: PgPool) -> Self {
        #[allow(clippy::expect_used)]
        let r = Regex::new(r"^(?:ingredients)_(?<field>.*)_(?:key)|(?<pkey>pkey)")
            .expect("Error in regex creation - this shouldn't happen like, ever.");
        Self(pool, r)
    }
}

#[cfg(test)]
mod tests;
