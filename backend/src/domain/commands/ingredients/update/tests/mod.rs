mod __tests__;

mod in_memory {
    use crate::domain::repositories::ingredients::in_memory::InMemoryIngredientRepository;

    use super::__tests__;

    #[tokio::test]
    async fn updating_an_ingredient_success() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::updating_an_ingredient_success(repo).await
    }

    #[tokio::test]
    async fn updating_with_empty_changeset_fails() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::updating_with_empty_changeset_fails(repo).await
    }

    #[tokio::test]
    async fn updating_a_missing_file_fails() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::updating_a_missing_file_fails(repo).await
    }
}

mod sql {
    use sqlx::PgPool;

    use super::__tests__;
    use crate::domain::repositories::ingredients::postgres::PostgresIngredientRepository;

    #[sqlx::test]
    async fn updating_an_ingredient_success(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());
        __tests__::updating_an_ingredient_success(repo).await
    }

    #[sqlx::test]
    async fn updating_with_empty_changeset_fails(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());
        __tests__::updating_with_empty_changeset_fails(repo).await
    }

    #[sqlx::test]
    async fn updating_a_missing_file_fails(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());
        __tests__::updating_a_missing_file_fails(repo).await
    }
}
