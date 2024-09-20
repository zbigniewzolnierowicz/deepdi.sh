use common::IngredientDTO;

pub fn recipe_fixture(ingredients: &[IngredientDTO]) -> serde_json::Value {
    serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "ingredients": ingredients
            .iter()
            .map(|ingredient| {
                serde_json::json!({
                    "ingredient_id": ingredient.id,
                    "optional": false,
                    "amount": {
                        "_type": "grams",
                        "amount": 100.0
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
    })
}
