mod __tests__;
mod in_memory {
    use super::__tests__;
    use crate::domain::repositories::ingredients::in_memory::InMemoryIngredientRepository;

    #[tokio::test]
    async fn creates_an_ingredient() {
        __tests__::creates_an_ingredient(InMemoryIngredientRepository::new()).await;
    }

    #[tokio::test]
    async fn incorrect_diets_do_not_get_included() {
        __tests__::incorrect_diets_do_not_get_included(InMemoryIngredientRepository::new()).await;
    }

    #[tokio::test]
    async fn empty_name_fails() {
        __tests__::empty_name_fails(InMemoryIngredientRepository::new()).await;
    }

    #[tokio::test]
    async fn empty_description_fails() {
        __tests__::empty_description_fails(InMemoryIngredientRepository::new()).await;
    }

    #[tokio::test]
    async fn incorrect_ingredient_is_not_persisted() {
        __tests__::incorrect_ingredient_is_not_persisted(InMemoryIngredientRepository::new()).await;
    }
}

mod sql {
    use super::__tests__;
    use crate::domain::repositories::ingredients::postgres::PostgresIngredientRepository;

    use sqlx::PgPool;

    #[sqlx::test]
    async fn creates_an_ingredient(pool: PgPool) {
        __tests__::creates_an_ingredient(PostgresIngredientRepository::new(pool)).await;
    }

    #[sqlx::test]
    async fn incorrect_diets_do_not_get_included(pool: PgPool) {
        __tests__::incorrect_diets_do_not_get_included(PostgresIngredientRepository::new(pool))
            .await;
    }

    #[sqlx::test]
    async fn empty_name_fails(pool: PgPool) {
        __tests__::empty_name_fails(PostgresIngredientRepository::new(pool)).await;
    }

    #[sqlx::test]
    async fn empty_description_fails(pool: PgPool) {
        __tests__::empty_description_fails(PostgresIngredientRepository::new(pool)).await;
    }

    #[sqlx::test]
    async fn incorrect_ingredient_is_not_persisted(pool: PgPool) {
        __tests__::incorrect_ingredient_is_not_persisted(PostgresIngredientRepository::new(pool))
            .await;
    }
}
