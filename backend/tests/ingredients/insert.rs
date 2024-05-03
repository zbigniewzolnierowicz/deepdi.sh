use common::IngredientDTO;
use reqwest::{Client, StatusCode};
use serde_json::json;

use crate::setup::TestApp;

#[tokio::test]
async fn inserting_ingredient_succeeds() {
    let app = TestApp::new().await;
    let client = Client::new();
    let path = app.get_base("ingredient/create");

    let request = client
        .post(&path)
        .json(&json!({
            "name": "Tomato",
            "description": "Tomatoes are very squishy",
            "diet_friendly": ["vegan", "vegetarian", "gluten_free"]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(request.status(), StatusCode::OK);

    let body = request.json::<IngredientDTO>().await.unwrap();

    let expected_body = IngredientDTO {
        id: uuid::Uuid::from_u128(0),
        name: "Tomato".to_string(),
        description: "Tomatoes are very squishy".to_string(),
        diet_friendly: vec!["vegan".to_string(), "vegetarian".to_string(), "gluten_free".to_string()],
    };

    assert_eq!(body.name, expected_body.name);
    assert_eq!(body.description, expected_body.description);
    assert_eq!(body.diet_friendly, expected_body.diet_friendly);
}

#[tokio::test]
async fn sending_insufficient_data_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let path = app.get_base("ingredient/create");

    let request = client
        .post(&path)
        .json(&json!({
            "description": "This is an example without the name",
            "diet_friendly": ["vegan", "vegetarian", "gluten_free"]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(request.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn incorrect_diets_are_ignored() {
    let app = TestApp::new().await;
    let client = Client::new();
    let path = app.get_base("ingredient/create");

    let request = client
        .post(&path)
        .json(&json!({
            "name": "Tomato",
            "description": "Tomatoes are very squishy",
            "diet_friendly": ["vegan", "vegetarian", "gluten_free", "I_AM_INCORRECT"]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(request.status(), StatusCode::OK);

    let body = request.json::<IngredientDTO>().await.unwrap();

    assert_eq!(body.diet_friendly, vec!["vegan".to_string(), "vegetarian".to_string(), "gluten_free".to_string()]);
}
