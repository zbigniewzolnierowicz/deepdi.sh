use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct CreateIngredientDTO {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diet_friendly: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, TS, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, ToSchema, TS)]
#[ts(export)]
pub struct UpdateIngredientDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diet_friendly: Option<Vec<String>>,
}
