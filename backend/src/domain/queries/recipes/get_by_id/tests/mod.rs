mod __tests__;

mod in_memory {

    use crate::domain::repositories::{
        ingredients::in_memory::InMemoryIngredientRepository,
        recipe::in_memory::InMemoryRecipeRepository,
    };

    use super::__tests__;

    #[tokio::test]
    async fn getting_recipe_by_id_works() {
        let repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();
        __tests__::getting_recipe_by_id_works(repo, ingredient_repo).await
    }

    #[tokio::test]
    async fn getting_a_nonexistent_recipe_errors() {
        let repo = InMemoryRecipeRepository::new();
        __tests__::getting_a_nonexistent_recipe_errors(repo).await
    }
}

mod sql {
    use sqlx::PgPool;

    use super::__tests__;
    use crate::domain::repositories::{
        ingredients::postgres::PostgresIngredientRepository,
        recipe::postgres::PostgresRecipeRepository,
    };

    #[sqlx::test]
    async fn getting_recipe_by_id_works(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool);
        __tests__::getting_recipe_by_id_works(repo, ingredient_repo).await
    }

    #[sqlx::test]
    async fn getting_a_nonexistent_recipe_errors(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        __tests__::getting_a_nonexistent_recipe_errors(repo).await
    }
}
