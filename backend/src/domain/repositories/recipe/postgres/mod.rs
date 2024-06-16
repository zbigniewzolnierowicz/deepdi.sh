use async_trait::async_trait;
use futures::future::join_all;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::ingredient::IngredientModel;
use crate::domain::entities::recipe::{
    IngredientWithAmount, IngredientWithAmountModel, Recipe, RecipeChangeset,
};

use super::errors::{DeleteRecipeError, UpdateRecipeError};
use super::{
    errors::{GetRecipeByIdError, InsertRecipeError},
    RecipeRepository,
};

pub struct PostgresRecipeRepository(pub PgPool);

async fn insert_ingredient(
    pool: &PgPool,
    id: Uuid,
    ingredient: &IngredientWithAmount,
) -> Result<(), InsertRecipeError> {
    let amount = serde_json::to_value(ingredient.amount.clone())
        .map_err(|e| InsertRecipeError::UnknownError(e.into()))?;

    sqlx::query_file!(
        "queries/recipes/insert_ingredient.sql",
        id,
        ingredient.ingredient.id,
        amount,
        ingredient.notes,
        ingredient.optional
    )
    .execute(pool)
    .await
    .map_err(InsertRecipeError::from)?;

    Ok(())
}

#[async_trait]
impl RecipeRepository for PostgresRecipeRepository {
    async fn insert(&self, input: Recipe) -> Result<Recipe, InsertRecipeError> {
        let time = serde_json::to_value(&input.time)
            .map_err(|e| InsertRecipeError::UnknownError(e.into()))?;

        let servings = serde_json::to_value(&input.servings)
            .map_err(|e| InsertRecipeError::UnknownError(e.into()))?;

        let tx = self.0.begin().await.map_err(InsertRecipeError::from)?;

        let result = sqlx::query_file!(
            "queries/recipes/insert_recipe.sql",
            input.id,
            input.name,
            input.description,
            &input.steps.as_ref(),
            time,
            servings,
            serde_json::json!({})
        )
        .fetch_one(&self.0)
        .await
        .map_err(InsertRecipeError::from)?;

        join_all(
            input
                .ingredients
                .as_ref()
                .iter()
                .map(|i| insert_ingredient(&self.0, result.id, i)),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<()>, InsertRecipeError>>()?;

        tx.commit().await.map_err(InsertRecipeError::from)?;

        self.get_by_id(&result.id)
            .await
            .map_err(InsertRecipeError::Get)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Recipe, GetRecipeByIdError> {
        let result = sqlx::query_file!("queries/recipes/get_recipe.sql", id)
            .fetch_one(&self.0)
            .await
            .map_err(|e| GetRecipeByIdError::with_id(id, e))?;

        let result_ingredients = sqlx::query_file_as!(
            IngredientWithAmountModel,
            "queries/recipes/get_ingredients_for_recipe.sql",
            id
        )
        .fetch_all(&self.0)
        .await
        .map_err(|e| GetRecipeByIdError::UnknownError(e.into()))?;

        let ingredients = result_ingredients
            .iter()
            .map(IngredientWithAmount::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(GetRecipeByIdError::from)?;

        let time = serde_json::from_value(result.time)
            .map_err(|e| GetRecipeByIdError::UnknownError(e.into()))?;

        let servings = serde_json::from_value(result.servings)
            .map_err(|e| GetRecipeByIdError::UnknownError(e.into()))?;

        let recipe = Recipe {
            id: result.id,
            name: result.name,
            description: result.description,
            steps: result.steps.try_into()?,
            time,
            servings,
            ingredients: ingredients.try_into()?,
        };

        Ok(recipe)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DeleteRecipeError> {
        let recipe = self.get_by_id(id).await?;

        let tx = self
            .0
            .begin()
            .await
            .map_err(|e| DeleteRecipeError::UnknownError(e.into()))?;

        sqlx::query_file!(
            "queries/recipes/delete_ingredients_for_recipe.sql",
            recipe.id
        )
        .execute(&self.0)
        .await
        .map_err(|e| DeleteRecipeError::UnknownError(e.into()))?;

        sqlx::query_file!("queries/recipes/delete_recipe.sql", recipe.id)
            .execute(&self.0)
            .await
            .map_err(|e| DeleteRecipeError::UnknownError(e.into()))?;

        tx.commit()
            .await
            .map_err(|e| DeleteRecipeError::UnknownError(e.into()))?;

        Ok(())
    }

    async fn update(
        &self,
        id: &Uuid,
        changeset: RecipeChangeset,
    ) -> Result<Recipe, UpdateRecipeError> {
        let recipe = self.get_by_id(id).await?;

        let tx = self
            .0
            .begin()
            .await
            .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;

        if let Some(value) = changeset.name {
            if value != recipe.name {
                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET name = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await
                .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;
            }
        };

        if let Some(value) = changeset.description {
            if value != recipe.description {
                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET description = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await
                .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;
            }
        };

        if let Some(value) = changeset.servings {
            if value != recipe.servings {
                let value = serde_json::to_value(value)
                    .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;

                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET servings = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await
                .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;
            }
        }

        if let Some(value) = changeset.time {
            if value != recipe.time {
                let value = serde_json::to_value(value)
                    .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;

                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET time = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await
                .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;
            }
        }

        if let Some(value) = changeset.steps {
            if value != recipe.steps {
                let value = value.as_ref();

                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET steps = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await
                .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;
            }
        }

        tx.commit()
            .await
            .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;

        let recipe = self.get_by_id(id).await?;

        Ok(recipe)
    }
}

impl PostgresRecipeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[cfg(test)]
mod tests;
