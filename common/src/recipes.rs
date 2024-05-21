use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::IngredientDTO;

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct CreateRecipeDTO {
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub time: HashMap<String, u64>,
    pub ingredients: Vec<IngredientAmountDTO>,
    pub servings: ServingsTypeDTO,
}

#[derive(Deserialize, Serialize, TS, Debug)]
#[ts(export)]
pub struct RecipeDTO {
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub time: HashMap<String, u64>,
    pub ingredients: Vec<IngredientWithAmountDTO>,
    pub servings: ServingsTypeDTO,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ServingsTypeDTO {
    FromTo(u16, u16),
    Exact(u16),
}

#[derive(Deserialize, Serialize, TS, Debug)]
#[ts(export)]
pub struct IngredientAmountDTO {
    pub ingredient_id: Uuid,
    pub amount: IngredientUnitDTO,
    pub optional: bool,
    pub notes: Option<String>,
}

#[derive(Deserialize, Serialize, TS, Debug)]
#[ts(export)]
pub struct IngredientWithAmountDTO {
    pub ingredient: IngredientDTO,
    pub amount: IngredientUnitDTO,
    pub optional: bool,
    pub notes: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum IngredientUnitDTO {
    Mililiters(f64),
    Grams(f64),
    Teaspoons(f64),
    Cups(f64),
    Other { amount: f64, unit: String },
}
