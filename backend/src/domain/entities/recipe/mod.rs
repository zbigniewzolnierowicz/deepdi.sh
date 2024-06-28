pub mod errors;
use std::collections::BTreeMap;

use common::{IngredientUnitDTO, IngredientWithAmountDTO, RecipeDTO, ServingsTypeDTO};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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
    pub steps: RecipeSteps,
    pub ingredients: RecipeIngredients,
    pub time: BTreeMap<String, std::time::Duration>,
    pub servings: ServingsType,
}
#[derive(Debug, Clone)]
pub struct RecipeIngredients(Vec<IngredientWithAmount>);

impl AsRef<[IngredientWithAmount]> for RecipeIngredients {
    fn as_ref(&self) -> &[IngredientWithAmount] {
        &self.0
    }
}

impl PartialEq for RecipeIngredients {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.0.clone();
        let mut b = other.0.clone();
        a.sort_by_key(|f| f.ingredient.id);
        b.sort_by_key(|f| f.ingredient.id);

        a.eq(&b)
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct RecipeSteps(Vec<String>);

impl AsRef<[String]> for RecipeSteps {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}

impl TryFrom<Vec<String>> for RecipeSteps {
    type Error = ValidationError;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        // Filter out empty steps
        let data: Vec<String> = value
            .to_owned()
            .par_iter()
            .filter(|step| !step.trim().is_empty())
            .cloned()
            .collect();

        if data.is_empty() {
            Err(ValidationError::EmptyField(vec!["steps"]))
        } else {
            Ok(Self(data.to_owned()))
        }
    }
}

impl TryFrom<&Vec<String>> for RecipeSteps {
    type Error = ValidationError;
    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        RecipeSteps::try_from(value.clone())
    }
}

impl TryFrom<Vec<IngredientWithAmount>> for RecipeIngredients {
    type Error = ValidationError;
    fn try_from(value: Vec<IngredientWithAmount>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ValidationError::EmptyField(vec!["steps"]))
        } else {
            Ok(Self(value.to_owned()))
        }
    }
}

impl TryFrom<Recipe> for RecipeDTO {
    type Error = ValidationError;
    fn try_from(value: Recipe) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.to_string(),
            ingredients: value
                .ingredients
                .as_ref()
                .iter()
                .map(|i| i.clone().into())
                .collect(),
            name: value.name,
            description: value.description,
            steps: value.steps.0,
            time: value
                .time
                .into_iter()
                .map(|(k, v)| (k, v.as_secs()))
                .collect(),
            servings: value.servings.into(),
        })
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServingsType {
    FromTo(u16, u16),
    Exact(u16),
}

impl From<ServingsType> for ServingsTypeDTO {
    fn from(value: ServingsType) -> Self {
        match value {
            ServingsType::Exact(a) => Self::Exact(a),
            ServingsType::FromTo(a, b) => Self::FromTo(a, b),
        }
    }
}

impl From<ServingsTypeDTO> for ServingsType {
    fn from(value: ServingsTypeDTO) -> Self {
        match value {
            ServingsTypeDTO::Exact(a) => Self::Exact(a),
            ServingsTypeDTO::FromTo(a, b) => Self::FromTo(a, b),
        }
    }
}

impl From<&ServingsTypeDTO> for ServingsType {
    fn from(value: &ServingsTypeDTO) -> Self {
        Self::from(value.clone())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct IngredientWithAmount {
    pub ingredient: Ingredient,
    pub amount: IngredientUnit,
    pub notes: Option<String>,
    pub optional: bool,
}

impl From<IngredientWithAmount> for IngredientWithAmountDTO {
    fn from(value: IngredientWithAmount) -> Self {
        Self {
            ingredient: value.ingredient.into(),
            optional: value.optional,
            notes: value.notes,
            amount: value.amount.into(),
        }
    }
}

#[derive(FromRow, PartialEq, Debug, Clone)]
pub struct IngredientWithAmountModel {
    pub ingredient: IngredientModel,
    pub amount: serde_json::Value,
    pub notes: Option<String>,
    pub optional: bool,
}

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

impl TryFrom<IngredientWithAmountModel> for IngredientWithAmount {
    type Error = ValidationError;
    fn try_from(value: IngredientWithAmountModel) -> Result<Self, Self::Error> {
        Self::try_from(&value)
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

impl From<IngredientUnitDTO> for IngredientUnit {
    fn from(value: IngredientUnitDTO) -> Self {
        match value {
            IngredientUnitDTO::Cups(amount) => Self::Cups(amount),
            IngredientUnitDTO::Grams(amount) => Self::Grams(amount),
            IngredientUnitDTO::Mililiters(amount) => Self::Mililiters(amount),
            IngredientUnitDTO::Teaspoons(amount) => Self::Teaspoons(amount),
            IngredientUnitDTO::Other { amount, unit } => Self::Other { amount, unit },
        }
    }
}

impl From<IngredientUnit> for IngredientUnitDTO {
    fn from(value: IngredientUnit) -> Self {
        match value {
            IngredientUnit::Cups(amount) => Self::Cups(amount),
            IngredientUnit::Grams(amount) => Self::Grams(amount),
            IngredientUnit::Mililiters(amount) => Self::Mililiters(amount),
            IngredientUnit::Teaspoons(amount) => Self::Teaspoons(amount),
            IngredientUnit::Other { amount, unit } => Self::Other { amount, unit },
        }
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

#[derive(Default)]
pub struct RecipeChangeset {
    pub name: Option<String>,
    pub description: Option<String>,
    pub steps: Option<RecipeSteps>,
    pub time: Option<BTreeMap<String, std::time::Duration>>,
    pub servings: Option<ServingsType>,
}

#[cfg(test)]
mod tests;
