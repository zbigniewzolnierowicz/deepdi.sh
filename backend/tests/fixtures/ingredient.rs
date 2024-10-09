pub fn ingredient_fixture() -> serde_json::Value {
    serde_json::json!({
        "name": "Cucumber",
        "description": "A cucumber description.",
        "diet_violations": []
    })
}

pub fn ingredient_fixture_meat() -> serde_json::Value {
    serde_json::json!({
        "name": "Beef",
        "description": "A cow had to die for this",
        "diet_violations": [
            "vegan",
            "vegetarian",
        ],
    })
}

pub fn ingredient_fixture_evil() -> serde_json::Value {
    serde_json::json!({
        "name": "Evil Fruit",
        "description": "It is very evil",
        "diet_violations": [
            "vegan",
            "gluten_free",
        ]
    })
}
