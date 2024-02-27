use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct CreateIngredientDTO {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct IngredientDTO {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct IngredientWithAmount {
    pub id: i32,
    pub unit: String,
    pub amount: f64,
    pub name: String,
}
