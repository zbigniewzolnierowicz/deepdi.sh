use shrinkwraprs::Shrinkwrap;
use strum::{EnumString, VariantNames};

use super::errors::ValidationError;

#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq)]
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

#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq)]
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

#[derive(VariantNames, EnumString, Debug, PartialEq, Eq, Clone, Copy)]
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
