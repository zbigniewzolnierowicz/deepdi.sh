mod __tests__;

mod in_memory {
    use crate::domain::repositories::{
        ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
    };

    use super::__tests__;

    #[tokio::test]
    async fn deleting_a_recipe_succeeds() {
        let repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();
        __tests__::deleting_a_recipe_succeeds(repo, ingredient_repo).await
    }

    #[tokio::test]
    async fn deleting_a_nonexistent_recipe_fails() {
        let repo = InMemoryRecipeRepository::new();
        __tests__::deleting_a_nonexistent_recipe_fails(repo).await
    }
}

mod sql {
    use sqlx::PgPool;

    use crate::domain::repositories::{
        ingredients::postgres::PostgresIngredientRepository,
        recipe::postgres::PostgresRecipeRepository,
    };

    use super::__tests__;

    #[sqlx::test]
    async fn deleting_a_recipe_succeeds(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        __tests__::deleting_a_recipe_succeeds(repo, ingredient_repo).await
    }

    #[sqlx::test]
    async fn deleting_a_nonexistent_recipe_fails(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        __tests__::deleting_a_nonexistent_recipe_fails(repo).await
    }
}
