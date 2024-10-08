use crate::setup::TestApp;
use backend::domain::entities::ingredient::{types::DietViolations, IngredientModel};
use common::IngredientDTO;
use uuid::Uuid;

#[tokio::test]
async fn getting_all_with_empty_database_returns_empty_array() {
    let app = TestApp::new().await;

    let path = app.get_base("ingredient");

    let request = reqwest::get(&path).await.unwrap();

    assert_eq!(request.status(), 200);

    let body = request.json::<Vec<()>>().await.unwrap();

    assert_eq!(body, vec![]);
}

#[tokio::test]
async fn getting_all_with_full_database_returns_in_array() {
    let app = TestApp::new().await;

    let ingredients: Vec<IngredientModel> = vec![IngredientModel {
        id: Uuid::from_u128(1),
        name: "Tomato".to_string(),
        description: "Very yummy tomato".to_string(),
        diet_violations: vec![
            DietViolations::Vegan.to_string(),
            DietViolations::Vegetarian.to_string(),
        ],
    }];

    let tx = app.db.begin().await.unwrap();

    for ingredient in ingredients.clone() {
        let diet_violations: Vec<String> = ingredient
            .clone()
            .diet_violations
            .into_iter()
            .map(|d| d.to_string())
            .collect();

        sqlx::query!(
            r#"
                INSERT INTO ingredients (id, name, description, diet_violations)
                VALUES ($1, $2, $3, $4)
            "#,
            ingredient.id,
            &ingredient.name,
            &ingredient.description,
            &diet_violations
        )
        .execute(&app.db)
        .await
        .unwrap();
    }

    tx.commit().await.unwrap();

    let path = app.get_base("ingredient");
    let request = reqwest::get(&path).await.unwrap();

    assert_eq!(request.status(), 200);

    let body = request.json::<Vec<IngredientDTO>>().await.unwrap();
    let check: Vec<IngredientDTO> = ingredients.into_iter().map(|i| i.into()).collect();

    assert_eq!(body, check);
}
