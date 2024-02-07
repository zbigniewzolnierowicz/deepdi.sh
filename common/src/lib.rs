use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub unit: String,
    pub amount: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub serves: u16,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<String>
}
