mod __tests__;

mod in_memory {
    use super::__tests__;
    use crate::domain::repositories::ingredients::in_memory::InMemoryIngredientRepository;

    #[tokio::test]
    async fn get_by_id_returns_ingredient() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::get_by_id_returns_ingredient(repo).await;
    }

    #[tokio::test]
    async fn get_by_id_returns_error_when_missing() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::get_by_id_returns_error_when_missing(repo).await;
    }
}

mod sql {
    use sqlx::PgPool;

    use super::__tests__;
    use crate::domain::repositories::ingredients::postgres::PostgresIngredientRepository;

    #[sqlx::test]
    async fn get_by_id_returns_ingredient(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool);
        __tests__::get_by_id_returns_ingredient(repo).await;
    }

    #[sqlx::test]
    async fn get_by_id_returns_error_when_missing(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool);
        __tests__::get_by_id_returns_error_when_missing(repo).await;
    }
}
