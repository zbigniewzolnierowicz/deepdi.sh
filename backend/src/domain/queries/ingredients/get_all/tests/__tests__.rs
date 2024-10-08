use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    entities::ingredient::{
        types::{DietViolations, IngredientDescription, IngredientName, WhichDiets},
        Ingredient,
    },
    queries::ingredients::get_all::get_all_ingredients,
    repositories::ingredients::{IngredientRepository, IngredientRepositoryService},
};

use pretty_assertions::assert_eq;

pub async fn returns_empty_vec_when_no_items_inside(repo: impl IngredientRepository) {
    // GIVEN
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    // WHEN
    let result = get_all_ingredients(repo).await.unwrap();

    // THEN
    assert_eq!(result, vec![]);
}

pub async fn returns_vec_of_items_inside(repo: impl IngredientRepository) {
    // GIVEN
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let given_1 = Ingredient {
        id: Uuid::now_v7(),
        name: IngredientName("Tomato".into()),
        description: IngredientDescription("Description of a tomato".into()),
        diet_violations: vec![DietViolations::Vegan, DietViolations::Vegetarian].into(),
    };

    let given_2 = Ingredient {
        id: Uuid::now_v7(),
        name: IngredientName("Meat fries".into()),
        description: IngredientDescription("Description of meat fries (whatever they are)".into()),
        diet_violations: WhichDiets::new(),
    };

    repo.insert(given_1.clone()).await.unwrap();
    repo.insert(given_2.clone()).await.unwrap();

    // WHEN
    let mut result = get_all_ingredients(repo).await.unwrap();
    result.sort_by_key(|k| k.id);

    let mut expected = vec![given_1, given_2];
    expected.sort_by_key(|k| k.id);

    // THEN
    assert_eq!(result, expected);
}
