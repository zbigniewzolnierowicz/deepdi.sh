mod __tests__;

mod in_memory {
    use super::__tests__;
    use crate::domain::repositories::{
        ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
    };

    #[tokio::test]
    async fn adding_an_ingredient_to_a_recipe_works() {
        let ingredient_repo = InMemoryIngredientRepository::new();
        let repo = InMemoryRecipeRepository::new();
        __tests__::adding_an_ingredient_to_a_recipe_works(repo, ingredient_repo).await
    }

    #[tokio::test]
    async fn adding_a_nonexistent_ingredient_to_a_recipe_errors() {
        let ingredient_repo = InMemoryIngredientRepository::new();
        let repo = InMemoryRecipeRepository::new();
        __tests__::adding_a_nonexistent_ingredient_to_a_recipe_errors(repo, ingredient_repo).await
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
    async fn adding_an_ingredient_to_a_recipe_works(pool: PgPool) {
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        let repo = PostgresRecipeRepository::new(pool);
        __tests__::adding_an_ingredient_to_a_recipe_works(repo, ingredient_repo).await
    }

    #[sqlx::test]
    async fn adding_a_nonexistent_ingredient_to_a_recipe_errors(pool: PgPool) {
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        let repo = PostgresRecipeRepository::new(pool);
        __tests__::adding_a_nonexistent_ingredient_to_a_recipe_errors(repo, ingredient_repo).await
    }
}
