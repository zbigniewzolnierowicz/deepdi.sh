use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::recipe::Recipe;

use super::{errors::RecipeRepositoryError, RecipeRepository};

pub struct PostgresRecipeRepository(pub PgPool);

#[async_trait]
impl RecipeRepository for PostgresRecipeRepository {
    async fn insert(
        &self,
        _input: Recipe,
    ) -> Result<Recipe, RecipeRepositoryError> {
        todo!();
    }
}

#[cfg(test)] mod tests;
