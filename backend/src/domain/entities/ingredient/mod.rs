pub mod errors;
pub mod types;

use sqlx::FromRow;
use uuid::Uuid;

use self::{
    errors::ValidationError,
    types::{DietFriendly, IngredientDescription, IngredientName, WhichDiets},
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
    pub diet_friendly: WhichDiets,
}

#[derive(FromRow, Debug, Clone, PartialEq, sqlx::Decode)]
pub struct IngredientModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub diet_friendly: Vec<String>,
}

impl TryFrom<&IngredientModel> for Ingredient {
    type Error = ValidationError;
    fn try_from(value: &IngredientModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name.clone().try_into()?,
            description: value.description.clone().try_into()?,
            diet_friendly: value.diet_friendly.clone().into(),
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
                .filter_map(|s| DietFriendly::try_from(s).ok())
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
            diet_friendly,
        }: Ingredient,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            diet_friendly: diet_friendly.into(),
        }
    }
}

impl From<IngredientModel> for common::IngredientDTO {
    fn from(value: IngredientModel) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
            description: value.description.to_string(),
            diet_friendly: value.diet_friendly,
        }
    }
}

#[derive(Debug, Default)]
pub struct IngredientChangeset {
    pub name: Option<IngredientName>,
    pub description: Option<IngredientDescription>,
    pub diet_friendly: Option<WhichDiets>,
}
