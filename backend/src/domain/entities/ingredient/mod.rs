pub mod errors;
pub mod types;

use uuid::Uuid;

use self::types::{DietFriendly, IngredientDescription, IngredientName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: IngredientName,
    pub description: IngredientDescription,
    pub diet_friendly: Vec<DietFriendly>,
}
