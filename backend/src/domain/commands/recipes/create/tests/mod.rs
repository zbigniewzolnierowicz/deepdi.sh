mod __tests__;
mod in_memory {
    use super::__tests__;

    use crate::domain::repositories::{
        ingredients::InMemoryIngredientRepository, recipe::in_memory::InMemoryRecipeRepository,
    };

    #[tokio::test]
    async fn create_recipe_with_proper_ingredients() {
        let ingredient_repo = InMemoryIngredientRepository::new();
        let recipe_repo = InMemoryRecipeRepository::new();

        __tests__::create_recipe_with_proper_ingredients(recipe_repo, ingredient_repo).await;
    }

    #[tokio::test]
    async fn create_recipe_without_proper_ingredients_errors() {
        let ingredient_repo = InMemoryIngredientRepository::new();
        let recipe_repo = InMemoryRecipeRepository::new();

        __tests__::create_recipe_without_proper_ingredients_errors(recipe_repo, ingredient_repo)
            .await;
    }

    #[tokio::test]
    async fn inserting_recipe_with_same_id_fails() {
        let repo = InMemoryRecipeRepository::new();
        let ingredient_repo = InMemoryIngredientRepository::new();
        __tests__::inserting_recipe_with_same_id_fails(repo, ingredient_repo).await
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
    async fn create_recipe_with_proper_ingredients(pool: PgPool) {
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        let recipe_repo = PostgresRecipeRepository::new(pool.clone());

        __tests__::create_recipe_with_proper_ingredients(recipe_repo, ingredient_repo).await;
    }

    #[sqlx::test]
    async fn create_recipe_without_proper_ingredients_errors(pool: PgPool) {
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        let recipe_repo = PostgresRecipeRepository::new(pool.clone());

        __tests__::create_recipe_without_proper_ingredients_errors(recipe_repo, ingredient_repo)
            .await;
    }

    #[sqlx::test]
    async fn inserting_recipe_with_same_id_fails(pool: PgPool) {
        let repo = PostgresRecipeRepository::new(pool.clone());
        let ingredient_repo = PostgresIngredientRepository::new(pool.clone());
        __tests__::inserting_recipe_with_same_id_fails(repo, ingredient_repo).await
    }
}
