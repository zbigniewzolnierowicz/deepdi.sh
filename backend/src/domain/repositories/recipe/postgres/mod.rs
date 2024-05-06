use async_trait::async_trait;
use sqlx::{Error as SQLXError, PgPool};
use std::{collections::HashMap, sync::OnceLock};

use crate::domain::entities::ingredient::IngredientModel;
use crate::domain::entities::recipe::{IngredientWithAmount, IngredientWithAmountModel, Recipe};

use super::{errors::RecipeRepositoryError, RecipeRepository};

pub struct PostgresRecipeRepository(pub PgPool);

/// Turns out Postgres doesn't return the column name for unique constraints isn't returned.
/// This function maps constraints to fields
fn constraint_to_field(field: &str) -> &str {
    static HASHMAP: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    let m = HASHMAP.get_or_init(|| {
        HashMap::from_iter([("ingredients_name_key", "name"), ("ingredients_pkey", "id")])
    });
    m.get(field).unwrap_or(&field)
}

fn map_error_to_internal(e: SQLXError) -> RecipeRepositoryError {
    match e {
        SQLXError::Database(dberror) if dberror.is_unique_violation() => {
            RecipeRepositoryError::Conflict(
                constraint_to_field(dberror.constraint().unwrap_or_default()).to_string(),
            )
        }
        e => RecipeRepositoryError::UnknownError(e.into()),
    }
}

#[async_trait]
impl RecipeRepository for PostgresRecipeRepository {
    async fn insert(&self, input: Recipe) -> Result<Recipe, RecipeRepositoryError> {
        let time = serde_json::to_value(&input.time)
            .map_err(|e| RecipeRepositoryError::UnknownError(e.into()))?;

        let servings = serde_json::to_value(&input.servings)
            .map_err(|e| RecipeRepositoryError::UnknownError(e.into()))?;

        let tx = self.0.begin().await.map_err(map_error_to_internal)?;

        let result = sqlx::query!(
            r#"INSERT INTO recipes
            (id, name, description, steps, time, servings, metadata)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id"#,
            input.id,
            input.name,
            input.description,
            &input.steps,
            time,
            servings,
            serde_json::json!({})
        )
        .fetch_one(&self.0)
        .await
        .map_err(map_error_to_internal)?;

        for ingredient in input.ingredients {
            let amount = serde_json::to_value(ingredient.amount)
                .map_err(|e| RecipeRepositoryError::UnknownError(e.into()))?;

            sqlx::query!(
                r#"
                INSERT INTO ingredients_recipes
                (recipe_id, ingredient_id, amount, notes, optional)
                VALUES
                ($1, $2, $3, $4, $5)
                "#,
                result.id,
                ingredient.ingredient.id,
                amount,
                ingredient.notes,
                ingredient.optional
            )
            .execute(&self.0)
            .await
            .map_err(map_error_to_internal)?;
        }

        tx.commit().await.map_err(map_error_to_internal)?;

        // TODO: There's got to be a way to turn this into a single query

        let inserted = sqlx::query_file!(
            "queries/get_recipe.sql",
            result.id
        )
        .fetch_one(&self.0)
        .await
        .map_err(map_error_to_internal)?;

        let inserted_ingredients = sqlx::query_file_as!(
            IngredientWithAmountModel,
            "queries/get_ingredients_for_recipe.sql",
            result.id
        )
        .fetch_all(&self.0)
        .await
        .map_err(map_error_to_internal)?;

        let ingredients = inserted_ingredients
                .iter()
                .map(IngredientWithAmount::try_from)
                .collect::<Result<Vec<_>, _>>()?;

        let recipe = Recipe {
            id: inserted.id,
            name: inserted.name,
            description: inserted.description,
            steps: inserted.steps,
            time: serde_json::from_value(inserted.time)
                .map_err(|e| RecipeRepositoryError::UnknownError(e.into()))?,
            servings: serde_json::from_value(inserted.servings)
                .map_err(|e| RecipeRepositoryError::UnknownError(e.into()))?,
            ingredients,
        };

        Ok(recipe)
    }
}

impl PostgresRecipeRepository {
    #[allow(dead_code)] // TODO: Remove after connecting to the API
    fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

#[cfg(test)]
mod tests;
