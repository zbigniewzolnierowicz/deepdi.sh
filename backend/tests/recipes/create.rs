use assert_json_diff::assert_json_include;
use common::{ingredients::IngredientDTO, RecipeDTO};
use reqwest::Client;

use crate::setup::TestApp;

#[tokio::test]
async fn inserts_recipe_correctly() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient/create");
    let recipe_create_path = app.get_base("recipe/create");

    let ingredients_input = vec![serde_json::json!({
        "name": "Cucumber",
        "description": "A cucumber description.",
        "diet_friendly": [
            "vegan",
            "vegetarian",
            "gluten_free"
        ]
    })];

    let mut ingredients: Vec<IngredientDTO> = vec![];

    for ingredient in ingredients_input {
        let result: IngredientDTO = client
            .post(&ingredient_create_path)
            .json(&ingredient)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        ingredients.push(result);
    }

    let data = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "ingredients": ingredients
            .iter()
            .map(|ingredient| {
                serde_json::json!({
                    "ingredient_id": ingredient.id,
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

    let expected = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "time": {
            "Prep time": 6000
        },
        "ingredients": ingredients
            .iter()
            .map(|ing| serde_json::json!({
                "ingredient": ing,
                "optional": false
            }))
            .collect::<Vec<_>>(),
        "steps": ["Get a cucumber", "Dice it"],
        "servings": {
            "exact": 1
        },
    });

    assert_eq!(result.ingredients.len(), 1);
    assert_json_include!(actual: result, expected: expected);
}

#[tokio::test]
async fn inserting_recipe_with_incorrect_ingredients_fails() {
    let app = TestApp::new().await;
    let client = Client::new();
    let recipe_create_path = app.get_base("recipe/create");

    let data = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "ingredients": [
            {
                "ingredient_id": uuid::Uuid::from_u128(0),
                "optional": false,
                "amount": {
                    "grams": 100.0
                },
            }
        ],
        "time": {
            "Prep time": 6000
        },
        "steps": ["Get a cucumber", "Dice it"],
        "servings": {
            "exact": 1
        },
    });

    let result = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap();

    let result = result
        .json::<common::error::ErrorMessageWithJsonValue>()
        .await
        .unwrap();

    assert_eq!(result.kind, "IngredientsNotFound");
}

#[tokio::test]
async fn inserting_recipe_with_partially_incorrect_ingredients() {
    let app = TestApp::new().await;
    let client = Client::new();
    let recipe_create_path = app.get_base("recipe/create");
    let ingredient_create_path = app.get_base("ingredient/create");

    let ingredient = serde_json::json!({
        "name": "Cucumber",
        "description": "A cucumber description.",
        "diet_friendly": [
            "vegan",
            "vegetarian",
            "gluten_free"
        ]
    });

    let ingredient_result: IngredientDTO = client
        .post(&ingredient_create_path)
        .json(&ingredient)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let data = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "ingredients": [
            {
                "ingredient_id": uuid::Uuid::from_u128(0),
                "optional": false,
                "amount": {
                    "grams": 100.0
                },
            },
            {
                "ingredient_id": ingredient_result.id,
                "optional": false,
                "amount": {
                    "grams": 100.0
                },
            }
        ],
        "time": {
            "Prep time": 6000
        },
        "steps": ["Get a cucumber", "Dice it"],
        "servings": {
            "exact": 1
        },
    });

    let result = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap();

    let result = result
        .json::<common::error::ErrorMessageWithJsonValue>()
        .await
        .unwrap();

    assert_eq!(result.kind, "IngredientsNotFound");
}
