use std::str::FromStr;

use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;
use strum::{Display, EnumString, VariantNames};

use super::errors::ValidationError;

#[derive(Serialize, Deserialize, Shrinkwrap, sqlx::Type, Debug, Clone, PartialEq, Eq, Hash)]
#[sqlx(transparent)]
pub struct IngredientName(pub String);

impl std::fmt::Display for IngredientName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// TODO: add doctest
impl TryFrom<String> for IngredientName {
    type Error = ValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError::EmptyField(vec!["name"]));
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for IngredientName {
    type Error = ValidationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[derive(Serialize, Deserialize, Shrinkwrap, sqlx::Type, Debug, Clone, PartialEq, Eq, Hash)]
#[sqlx(transparent)]
pub struct IngredientDescription(pub String);

// TODO: add doctest
impl TryFrom<String> for IngredientDescription {
    type Error = ValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError::EmptyField(vec!["description"]));
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for IngredientDescription {
    type Error = ValidationError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl std::fmt::Display for IngredientDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(
    Serialize,
    Deserialize,
    VariantNames,
    sqlx::Type,
    EnumString,
    Display,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
)]
#[strum(serialize_all = "snake_case")]
pub enum DietViolations {
    Vegan,
    Vegetarian,
    GlutenFree,
}

impl TryFrom<String> for DietViolations {
    type Error = ValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "vegan" => Ok(Self::Vegan),
            "vegetarian" => Ok(Self::Vegetarian),
            "gluten_free" => Ok(Self::GlutenFree),
            _ => Err(ValidationError::DoesNotMatch(
                "diet_violations",
                Self::VARIANTS,
            )),
        }
    }
}

#[derive(
    Serialize, Deserialize, Shrinkwrap, sqlx::Type, sqlx::FromRow, PartialEq, Eq, Clone, Debug,
)]
pub struct WhichDiets(pub Vec<DietViolations>);

impl WhichDiets {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Default for WhichDiets {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: add doctest
impl From<Vec<String>> for WhichDiets {
    fn from(value: Vec<String>) -> Self {
        Self(
            value
                .iter()
                .filter_map(|v| DietViolations::from_str(v).ok())
                .collect(),
        )
    }
}

impl From<Vec<DietViolations>> for WhichDiets {
    fn from(value: Vec<DietViolations>) -> Self {
        Self(value)
    }
}

impl From<WhichDiets> for Vec<String> {
    fn from(val: WhichDiets) -> Self {
        val.0.iter().map(|d| d.to_string()).collect()
    }
}

impl PartialEq<WhichDiets> for Vec<String> {
    fn eq(&self, other: &WhichDiets) -> bool {
        self.eq(&other.0)
    }
}

impl PartialEq<DietViolations> for String {
    fn eq(&self, other: &DietViolations) -> bool {
        self == &other.to_string()
    }
}
