pub mod errors;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use self::errors::ValidationError;

use super::ingredient::{Ingredient, IngredientModel};

#[derive(PartialEq, Debug, Clone)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    // TODO: add newtype for checking if the steps list is not empty
    pub steps: Vec<String>,
    // TODO: add newtype for checking if the ingredients list is not empty
    pub ingredients: Vec<IngredientWithAmount>,
    pub time: HashMap<String, std::time::Duration>,
    pub servings: ServingsType,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServingsType {
    FromTo(u16, u16),
    Exact(u16),
}

#[derive(PartialEq, Debug, Clone)]
pub struct IngredientWithAmount {
    pub ingredient: Ingredient,
    pub amount: IngredientUnit,
    pub notes: Option<String>,
    pub optional: bool,
}

#[derive(FromRow, PartialEq, Debug, Clone)]
pub struct IngredientWithAmountModel {
    pub ingredient: IngredientModel,
    pub amount: serde_json::Value,
    pub notes: Option<String>,
    pub optional: bool,
}

// TODO: write doctests
// TODO: make more graceful errors
impl TryFrom<&IngredientWithAmountModel> for IngredientWithAmount {
    type Error = ValidationError;
    fn try_from(value: &IngredientWithAmountModel) -> Result<Self, Self::Error> {
        Ok(Self {
            optional: value.optional,
            notes: value.notes.clone(),
            amount: serde_json::from_value(value.amount.clone())
                .map_err(|e| ValidationError::DeserializationFailed("amount", e))?,
            ingredient: value.ingredient.clone().try_into()?,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IngredientUnit {
    Mililiters(f64),
    Grams(f64),
    Teaspoons(f64),
    Cups(f64),
    Other { amount: f64, unit: String },
}

impl Default for IngredientUnit {
    fn default() -> Self {
        Self::Grams(0.0)
    }
}

impl IngredientUnit {
    /// Converts tablespoons to teaspoons
    /// 1 tbsp = 3 tsp
    /// ```rust
    /// use crate::backend::domain::entities::recipe::IngredientUnit;
    ///
    /// assert_eq!(IngredientUnit::from_tablespoons(4.0), IngredientUnit::Teaspoons(12.0))
    /// ```
    pub fn from_tablespoons(tablespoons: f64) -> Self {
        Self::Teaspoons(tablespoons * 3.0)
    }
}
