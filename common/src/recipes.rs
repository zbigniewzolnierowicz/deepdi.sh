use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::IngredientWithAmount;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RecipeDTO {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub serves: u16,
    pub ingredients: Vec<IngredientWithAmount>,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateRecipeDTO {
    pub name: String,
    pub description: String,
    pub serves: u16,
    pub ingredients: Vec<CreateRecipeIngredient>,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct CreateRecipeIngredient {
    pub id: i32,
    pub amount: f64,
    pub unit: String,
}
