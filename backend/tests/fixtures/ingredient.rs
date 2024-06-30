pub fn ingredient_fixture() -> serde_json::Value {
    serde_json::json!({
        "name": "Cucumber",
        "description": "A cucumber description.",
        "diet_friendly": [
            "vegan",
            "vegetarian",
            "gluten_free"
        ]
    })
}

pub fn ingredient_fixture_meat() -> serde_json::Value {
    serde_json::json!({
        "name": "Beef",
        "description": "A cow had to die for this",
        "diet_friendly": [],
    })
}
