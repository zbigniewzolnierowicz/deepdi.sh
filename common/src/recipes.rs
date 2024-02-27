use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::IngredientWithAmount;

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct RecipeDTO {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<IngredientWithAmount>,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct BareRecipeDTO {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct CreateRecipeDTO {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<CreateRecipeIngredient>,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, TS)]
#[ts(export)]
pub struct CreateRecipeIngredient {
    pub id: i32,
    pub amount: f64,
    pub unit: String,
}
