pub mod __tests__;

mod in_memory {
    use super::__tests__;
    use crate::domain::repositories::{
        ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
    };

    #[tokio::test]
    async fn updating_ingredient_in_recipe_works() {
        let recipe_repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();

        __tests__::updating_ingredient_in_recipe_works(recipe_repo, ingredient_repo).await
    }

    #[tokio::test]
    async fn updating_ingredient_in_nonexistent_recipe_errors() {
        let recipe_repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();

        __tests__::updating_ingredient_in_nonexistent_recipe_errors(recipe_repo, ingredient_repo)
            .await
    }

    #[tokio::test]
    async fn updating_nonexistent_ingredient_in_recipe_errors() {
        let recipe_repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();

        __tests__::updating_nonexistent_ingredient_in_recipe_errors(recipe_repo, ingredient_repo)
            .await
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
    async fn updating_ingredient_in_recipe_works(pool: PgPool) {
        let recipe_repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());

        __tests__::updating_ingredient_in_recipe_works(recipe_repo, ingredient_repo).await
    }

    #[sqlx::test]
    async fn updating_ingredient_in_nonexistent_recipe_errors(pool: PgPool) {
        let recipe_repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());

        __tests__::updating_ingredient_in_nonexistent_recipe_errors(recipe_repo, ingredient_repo)
            .await
    }

    #[sqlx::test]
    async fn updating_nonexistent_ingredient_in_recipe_errors(pool: PgPool) {
        let recipe_repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());

        __tests__::updating_nonexistent_ingredient_in_recipe_errors(recipe_repo, ingredient_repo)
            .await
    }
}
