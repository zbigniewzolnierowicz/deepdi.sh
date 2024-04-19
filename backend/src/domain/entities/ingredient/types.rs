use std::str::FromStr;

use shrinkwraprs::Shrinkwrap;
use strum::{Display, EnumString, VariantNames};

use super::errors::ValidationError;

#[derive(Shrinkwrap, sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct IngredientName(pub String);

impl TryFrom<String> for IngredientName {
    type Error = ValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError::EmptyField("name"));
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

impl ToString for IngredientName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Shrinkwrap, sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct IngredientDescription(pub String);

impl TryFrom<String> for IngredientDescription {
    type Error = ValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ValidationError::EmptyField("description"));
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

impl ToString for IngredientDescription {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(VariantNames, sqlx::Type, EnumString, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DietFriendly {
    Vegan,
    Vegetarian,
    GlutenFree,
}

impl TryFrom<String> for DietFriendly {
    type Error = ValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Vegan" => Ok(Self::Vegan),
            "Vegetarian" => Ok(Self::Vegetarian),
            "GlutenFree" => Ok(Self::GlutenFree),
            _ => Err(ValidationError::DoesNotMatch(
                "diet_friendly",
                Self::VARIANTS,
            )),
        }
    }
}

#[derive(Shrinkwrap, sqlx::Type, sqlx::FromRow, PartialEq, Eq, Clone, Debug)]
pub struct WhichDiets(pub Vec<DietFriendly>);

impl From<Vec<String>> for WhichDiets {
    fn from(value: Vec<String>) -> Self {
        Self(
            value
                .iter()
                .filter_map(|v| DietFriendly::from_str(v).ok())
                .collect(),
        )
    }
}

impl From<Vec<DietFriendly>> for WhichDiets {
    fn from(value: Vec<DietFriendly>) -> Self {
        Self(value)
    }
}

impl From<WhichDiets> for Vec<String> {
    fn from(val: WhichDiets) -> Self {
        val.0.iter().map(|d| d.to_string()).collect()
    }
}
