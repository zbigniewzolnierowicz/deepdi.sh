mod __tests__;

mod in_memory {
    use crate::domain::repositories::{
        ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
    };

    #[tokio::test]
    async fn deleting_an_existing_ingredient_works() {
        let repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();
        super::__tests__::deleting_an_existing_ingredient_works(repo, ingredient_repo).await;
    }

    #[tokio::test]
    async fn deleting_an_ingredient_that_doesnt_appear_in_recipe_errors() {
        let repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();
        super::__tests__::deleting_an_ingredient_that_doesnt_appear_in_recipe_errors(
            repo,
            ingredient_repo,
        )
        .await;
    }

    #[tokio::test]
    async fn deleting_an_ingredient_in_recipe_that_doesnt_exist_errors() {
        let repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();
        super::__tests__::deleting_an_ingredient_in_recipe_that_doesnt_exist_errors(
            repo,
            ingredient_repo,
        )
        .await;
    }

    #[tokio::test]
    async fn deleting_the_last_ingredient_in_recipe_errors() {
        let repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();
        super::__tests__::deleting_the_last_ingredient_in_recipe_errors(repo, ingredient_repo)
            .await;
    }
}

mod sql {
    use sqlx::PgPool;

    use crate::domain::repositories::{
        ingredients::postgres::PostgresIngredientRepository,
        recipe::postgres::PostgresRecipeRepository,
    };

    #[sqlx::test]
    async fn deleting_an_existing_ingredient_works(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        super::__tests__::deleting_an_existing_ingredient_works(repo, ingredient_repo).await;
    }

    #[sqlx::test]
    async fn deleting_an_ingredient_that_doesnt_appear_in_recipe_errors(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        super::__tests__::deleting_an_ingredient_that_doesnt_appear_in_recipe_errors(
            repo,
            ingredient_repo,
        )
        .await;
    }

    #[sqlx::test]
    async fn deleting_an_ingredient_in_recipe_that_doesnt_exist_errors(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        super::__tests__::deleting_an_ingredient_in_recipe_that_doesnt_exist_errors(
            repo,
            ingredient_repo,
        )
        .await;
    }

    #[sqlx::test]
    async fn deleting_the_last_ingredient_in_recipe_errors(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        super::__tests__::deleting_the_last_ingredient_in_recipe_errors(repo, ingredient_repo)
            .await;
    }
}
