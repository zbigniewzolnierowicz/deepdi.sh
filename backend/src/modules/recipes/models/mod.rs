use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: String
}

impl Ingredient {
    pub fn with_units_and_amount(&self, unit: &str, amount: f64) -> IngredientInRecipe {
        IngredientInRecipe {
            ingredient_id: self.id,
            unit: unit.to_string(),
            amount
        }
    }
}

pub struct IngredientInRecipe {
    pub ingredient_id: i32,
    pub amount: f64,
    pub unit: String
}

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub serves: u16,
    pub steps: Vec<String>,
    pub ingredients: Vec<Ingredient>
}
