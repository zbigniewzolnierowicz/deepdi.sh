use crate::{
    domain::repositories::ingredients::{
        postgres::PostgresIngredientRepository, IngredientRepository,
    },
    test_utils::recipe_fixture,
};

use super::*;

#[sqlx::test]
async fn creating_recipe_works(pool: PgPool) {
    let repo = PostgresRecipeRepository::new(pool.clone());
    let ingredient_repo = PostgresIngredientRepository::new(pool);

    let recipe = recipe_fixture();
    for ir in recipe.ingredients.clone() {
        ingredient_repo
            .insert(ir.ingredient)
            .await
            .expect("Could not insert an ingredient due to an error somewhere.");
    }
    let result = repo.insert(recipe.clone()).await.unwrap();
    assert_eq!(recipe, result);
}

// #[sqlx::test]
// async fn inserting_recipe_with_same_id_fails(pool: PgPool) {
//     let repo = PostgresRecipeRepository::new(pool);
//
//     let recipe = recipe_fixture();
//
//     repo.insert(recipe.clone()).await.unwrap();
//
//     let error = repo.insert(recipe.clone()).await.unwrap_err();
//
//     assert!(matches!(error, RecipeRepositoryError::Conflict(a) if a == "id"));
// }
