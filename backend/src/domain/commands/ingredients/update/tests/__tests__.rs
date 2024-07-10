use std::sync::Arc;

use uuid::Uuid;

use crate::{
    domain::{
        commands::ingredients::update::{
            update_ingredient, UpdateIngredient, UpdateIngredientError,
        },
        entities::ingredient::{
            errors::ValidationError,
            types::{IngredientName, WhichDiets},
            Ingredient,
        },
        repositories::ingredients::{IngredientRepository, IngredientRepositoryService},
    },
    test_utils::ingredient_fixture,
};

pub async fn updating_an_ingredient_success(repo: impl IngredientRepository) {
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    let input = ingredient_fixture();
    let changeset = UpdateIngredient {
        name: Some("Ingredient name changed".to_string()),
        ..Default::default()
    };

    repo.insert(input.clone()).await.unwrap();

    update_ingredient(repo.clone(), input.id, &changeset)
        .await
        .unwrap();

    let result = repo.get_by_id(&input.id).await.unwrap();

    assert_eq!(
        result,
        Ingredient {
            name: IngredientName("Ingredient name changed".to_string()),
            ..input
        }
    )
}

pub async fn updating_with_empty_changeset_fails(repo: impl IngredientRepository) {
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    let input = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name 1".try_into().unwrap(),
        description: "Ingredient description 1".try_into().unwrap(),
        diet_friendly: WhichDiets::new(),
    };
    let changeset = UpdateIngredient::default();

    repo.insert(input.clone()).await.unwrap();

    let error = update_ingredient(repo.clone(), input.id, &changeset)
        .await
        .unwrap_err();

    assert!(
        matches!(error, UpdateIngredientError::ValidationError(ValidationError::EmptyField(fields)) if fields == ["name", "description", "diet_friendly"])
    );
}

pub async fn updating_a_missing_file_fails(repo: impl IngredientRepository) {
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let input = ingredient_fixture();
    let changeset = UpdateIngredient {
        name: Some("This will fail, so this doesn't matter".to_string()),
        ..Default::default()
    };

    let error = update_ingredient(repo.clone(), input.id, &changeset)
        .await
        .unwrap_err();

    assert!(matches!(error, UpdateIngredientError::NotFound(id) if id == Uuid::from_u128(64)));
}
