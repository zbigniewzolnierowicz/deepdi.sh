use std::sync::Arc;

use crate::domain::{
    commands::ingredients::create::{create_ingredient, CreateIngredient, CreateIngredientError}, entities::ingredient::types::DietFriendly, repositories::ingredients::{IngredientRepository, IngredientRepositoryService}
};

pub async fn creates_an_ingredient(repo: impl IngredientRepository) {
    let given = CreateIngredient {
        name: "Tomato",
        description: "Description of a tomato",
        diet_friendly: vec!["Vegan".into()],
    };
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    let when = create_ingredient(repo.clone(), &given).await.unwrap();

    // THEN

    assert_eq!(when.name.as_ref(), "Tomato");
    assert_eq!(when.description.as_ref(), "Description of a tomato");
    assert!(when.diet_friendly.contains(&DietFriendly::Vegan));
}

pub async fn incorrect_diets_do_not_get_included(repo: impl IngredientRepository) {
    let given = CreateIngredient {
        name: "Tomato",
        description: "Description of a tomato",
        diet_friendly: vec!["Vegan".into(), "INVALID DIET".into()],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    let when = create_ingredient(repo.clone(), &given).await.unwrap();

    // THEN

    assert!(when.diet_friendly.contains(&DietFriendly::Vegan));
    assert_eq!(when.diet_friendly.len(), 1);
}

pub async fn empty_name_fails(repo: impl IngredientRepository) {
    let given = CreateIngredient {
        name: "",
        description: "Description of a tomato",
        diet_friendly: vec![],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    let when = create_ingredient(repo.clone(), &given).await.unwrap_err();

    // THEN

    assert!(matches!(when, CreateIngredientError::EmptyField("name")));
}

pub async fn empty_description_fails(repo: impl IngredientRepository) {
    let given = CreateIngredient {
        name: "Tomato",
        description: "",
        diet_friendly: vec![],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    let when = create_ingredient(repo.clone(), &given).await.unwrap_err();

    // THEN

    assert!(matches!(
        when,
        CreateIngredientError::EmptyField("description")
    ));
}

pub async fn incorrect_ingredient_is_not_persisted(repo: impl IngredientRepository) {
    let given = CreateIngredient {
        name: "",
        description: "Description of a tomato",
        diet_friendly: vec![],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));

    let when = create_ingredient(repo.clone(), &given).await.unwrap_err();

    // THEN

    assert!(matches!(when, CreateIngredientError::EmptyField(_)));

    assert!(!&repo
        .get_all()
        .await
        .unwrap()
        .into_iter()
        .any(|x| x.name.as_str() == given.name))
}
