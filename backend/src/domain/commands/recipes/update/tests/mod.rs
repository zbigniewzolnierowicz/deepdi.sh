mod __tests__;
mod in_memory {
    use crate::domain::repositories::{
        ingredients::in_memory::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
    };

    use super::__tests__;

    #[tokio::test]
    async fn updating_a_recipe_succeeds() {
        let ingredient_repo = InMemoryIngredientRepository::new();
        let repo = InMemoryRecipeRepository::new();
        __tests__::updating_a_recipe_succeeds(repo, ingredient_repo).await
    }

    #[tokio::test]
    async fn updating_a_nonexistent_recipe_fails() {
        let repo = InMemoryRecipeRepository::new();
        __tests__::updating_a_nonexistent_recipe_fails(repo).await
    }

    #[tokio::test]
    async fn updating_a_recipe_with_empty_changeset_does_nothing() {
        let ingredient_repo = InMemoryIngredientRepository::new();
        let repo = InMemoryRecipeRepository::new();
        __tests__::updating_a_recipe_with_empty_changeset_errors(repo, ingredient_repo).await
    }
}

mod sql {
    use super::__tests__;
    use sqlx::PgPool;

    use crate::domain::repositories::{
        ingredients::postgres::PostgresIngredientRepository,
        recipe::postgres::PostgresRecipeRepository,
    };

    #[sqlx::test]
    async fn updating_a_recipe_succeeds(pool: PgPool) {
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        let repo = PostgresRecipeRepository::new(pool);
        __tests__::updating_a_recipe_succeeds(repo, ingredient_repo).await
    }

    #[sqlx::test]
    async fn updating_a_nonexistent_recipe_fails(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool);
        __tests__::updating_a_nonexistent_recipe_fails(repo).await
    }

    #[sqlx::test]
    async fn updating_a_recipe_with_empty_changeset_does_nothing(pool: PgPool) {
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        let repo = PostgresRecipeRepository::new(pool);
        __tests__::updating_a_recipe_with_empty_changeset_errors(repo, ingredient_repo).await
    }
}
