use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct CreateIngredientDTO {
    pub name: String,
    pub description: String,
    pub diet_friendly: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct IngredientDTO {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct IngredientWithAmount {
    pub id: Uuid,
    pub unit: String,
    pub amount: f64,
    pub name: String,
}
