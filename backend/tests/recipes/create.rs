use common::{ingredients::IngredientDTO, RecipeDTO};
use reqwest::Client;

use crate::setup::TestApp;

#[tokio::test]
async fn inserts_recipe_correctly() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient/create");
    let recipe_create_path = app.get_base("recipe/create");

    let ingredients = vec![serde_json::json!({
        "name": "Cucumber",
        "description": "A cucumber description.",
        "diet_friendly": [
            "vegan",
            "vegetarian",
            "gluten_free"
        ]
    })];

    let mut ingredient_ids: Vec<uuid::Uuid> = vec![];

    for ingredient in ingredients {
        let result: IngredientDTO = client
            .post(&ingredient_create_path)
            .json(&ingredient)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        ingredient_ids.push(result.id);
    }

    let data = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "ingredients": ingredient_ids
            .iter()
            .map(|id| {
                serde_json::json!({
                    "ingredient_id": *id,
                    "optional": false,
                    "amount": {
                        "grams": 100.0
                    },
                })
            })
            .collect::<Vec<_>>(),
        "time": {
            "Prep time": 6000
        },
        "steps": ["Get a cucumber", "Dice it"],
        "servings": {
            "exact": 1
        },
    });

    let result: RecipeDTO = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(result.name, "A diced cucumber")
    // TODO: use assert_json_includes from https://crates.io/crates/assert-json-diff
}

#[tokio::test]
async fn inserting_recipe_with_incorrect_ingredients_fails() {
    todo!()
}

#[tokio::test]
async fn inserting_recipe_with_partially_incorrect_ingredients() {
    todo!()
}
