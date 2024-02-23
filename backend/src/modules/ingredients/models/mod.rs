use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl From<Ingredient> for common::Ingredient {
    fn from(val: Ingredient) -> Self {
        common::Ingredient {
            id: val.id,
            name: val.name,
            description: val.description
        }
    }
}
