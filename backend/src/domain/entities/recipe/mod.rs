pub mod errors;
use std::collections::HashMap;

use uuid::Uuid;

use super::ingredient::Ingredient;

#[derive(PartialEq, Debug, Clone)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub ingredients: Vec<IngredientWithAmount>,
    pub time: HashMap<String, std::time::Duration>,
    pub servings: ServingsType,
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
pub enum IngredientUnit {
    Mililiters(f64),
    Grams(f64),
    Teaspoons(f64),
    Cup(f64),
    Other(String, f64),
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

/* pub struct CreateRecipe {
    name: String,
    description: String,
    steps: Vec<String>,
    ingredients: Vec<IngredientWithAmount>,
    time: HashMap<String, std::time::Duration>,
    servings: ServingsType,
} */
