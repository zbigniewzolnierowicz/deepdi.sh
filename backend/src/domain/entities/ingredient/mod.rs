pub mod errors;
pub mod types;

use common::IngredientDTO;
use sqlx::FromRow;
use uuid::Uuid;

use self::{
    errors::ValidationError,
    types::{DietViolations, IngredientDescription, IngredientName, WhichDiets},
};

// TODO: Consider ingredients that are variants of other ingredients
// i.e.: soy sauce (dark, light)

// TODO: Include possible substitutes
#[derive(FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: IngredientName,
    pub description: IngredientDescription,
    // TODO: change to diet_violations for easier filtering
    pub diet_violations: WhichDiets,
}

impl From<Ingredient> for IngredientDTO {
    fn from(value: Ingredient) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
            description: value.description.to_string(),
            diet_violations: value.diet_violations.clone().into(),
        }
    }
}

impl From<&Ingredient> for IngredientDTO {
    fn from(value: &Ingredient) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
            description: value.description.to_string(),
            diet_violations: value.diet_violations.clone().into(),
        }
    }
}

#[derive(FromRow, Debug, Clone, PartialEq, sqlx::Decode)]
pub struct IngredientModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub diet_violations: Vec<String>,
}

impl TryFrom<&IngredientModel> for Ingredient {
    type Error = ValidationError;
    fn try_from(value: &IngredientModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name.clone().try_into()?,
            description: value.description.clone().try_into()?,
            diet_violations: value.diet_violations.clone().into(),
        })
    }
}

impl TryFrom<IngredientModel> for Ingredient {
    type Error = ValidationError;
    fn try_from(value: IngredientModel) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl FromIterator<String> for WhichDiets {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .filter_map(|s| DietViolations::try_from(s).ok())
                .collect(),
        )
    }
}

impl From<Ingredient> for IngredientModel {
    fn from(
        Ingredient {
            id,
            name,
            description,
            diet_violations,
        }: Ingredient,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            diet_violations: diet_violations.into(),
        }
    }
}

impl From<IngredientModel> for common::IngredientDTO {
    fn from(value: IngredientModel) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
            description: value.description.to_string(),
            diet_violations: value.diet_violations,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct IngredientChangeset {
    pub name: Option<IngredientName>,
    pub description: Option<IngredientDescription>,
    pub diet_violations: Option<WhichDiets>,
}
