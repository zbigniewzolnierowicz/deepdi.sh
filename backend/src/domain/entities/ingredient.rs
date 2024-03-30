use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Shrinkwrap, Debug, Clone)]
pub struct IngredientName(pub String);

#[derive(Shrinkwrap, Debug, Clone)]
pub struct IngredientDescription(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DietFriendly {
    Vegan,
    Vegetarian,
    GlutenFree,
}

#[derive(Clone)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: IngredientName,
    pub description: IngredientDescription,
    pub diet_friendly: Vec<DietFriendly>,
}
