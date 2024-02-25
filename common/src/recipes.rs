use serde::{Deserialize, Serialize};

use crate::IngredientWithAmount;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub serves: u16,
    pub ingredients: Vec<IngredientWithAmount>,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRecipe {
    pub name: String,
    pub description: String,
    pub serves: u16,
    pub ingredients: Vec<CreateRecipeIngredient>,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRecipeIngredient {
    pub id: i32,
    pub amount: f64,
    pub unit: String,
}
