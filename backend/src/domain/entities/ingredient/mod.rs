pub mod errors;
pub mod types;

use sqlx::FromRow;
use uuid::Uuid;

use self::{
    errors::ValidationError,
    types::{DietFriendly, IngredientDescription, IngredientName, WhichDiets},
};

#[derive(FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: IngredientName,
    pub description: IngredientDescription,
    pub diet_friendly: WhichDiets,
}

#[derive(FromRow, Debug, Clone)]
pub struct IngredientModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub diet_friendly: WhichDiets,
}

impl TryFrom<IngredientModel> for Ingredient {
    type Error = ValidationError;
    fn try_from(value: IngredientModel) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: value.name.try_into()?,
            description: value.description.try_into()?,
            diet_friendly: value.diet_friendly.into(),
        })
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
            diet_friendly: diet_friendly.into()
        }
    }
}
