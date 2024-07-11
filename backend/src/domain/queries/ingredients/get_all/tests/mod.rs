mod __tests__;
mod in_memory {
    use crate::domain::repositories::ingredients::in_memory::InMemoryIngredientRepository;

    use super::__tests__;

    #[tokio::test]
    async fn returns_empty_vec_when_no_items_inside() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::returns_empty_vec_when_no_items_inside(repo).await;
    }

    #[tokio::test]
    async fn returns_vec_of_items_inside() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::returns_vec_of_items_inside(repo).await;
    }
}

mod sql {
    use super::__tests__;
    use sqlx::PgPool;

    use crate::domain::repositories::ingredients::postgres::PostgresIngredientRepository;

    #[sqlx::test]
    async fn returns_empty_vec_when_no_items_inside(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool);
        __tests__::returns_empty_vec_when_no_items_inside(repo).await;
    }

    #[sqlx::test]
    async fn returns_vec_of_items_inside(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool);
        __tests__::returns_vec_of_items_inside(repo).await;
    }
}
