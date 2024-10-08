use std::collections::BTreeMap;

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
    #[ts(type = "Record<string, number>")]
    pub time: BTreeMap<String, u64>,
    pub ingredients: Vec<IngredientAmountDTO>,
    pub servings: ServingsTypeDTO,
}

#[derive(Deserialize, Serialize, TS, Debug)]
#[ts(export)]
pub struct RecipeDTO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    #[ts(type = "Record<string, number>")]
    pub time: BTreeMap<String, u64>,
    pub ingredients: Vec<IngredientWithAmountDTO>,
    pub servings: ServingsTypeDTO,
    pub created_at: String,
    pub updated_at: String,
    pub diet_violations: Vec<String>,
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
#[serde(tag = "_type", content = "amount", rename_all = "snake_case")]
#[ts(export)]
pub enum IngredientUnitDTO {
    Mililiters(f64),
    Grams(f64),
    Teaspoons(f64),
    Cups(f64),
    Other { amount: f64, unit: String },
}

#[derive(Deserialize, Serialize, TS, Debug)]
#[ts(export)]
pub struct UpdateRecipeDTO {
    pub name: Option<String>,
    pub description: Option<String>,
    pub steps: Option<Vec<String>>,
    #[ts(type = "Record<string, number> | null")]
    pub time: Option<BTreeMap<String, u64>>,
    pub servings: Option<ServingsTypeDTO>,
}
