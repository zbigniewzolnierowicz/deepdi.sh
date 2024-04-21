use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::IngredientDTO;

use crate::{
    api::AppState,
    domain::{
        entities::ingredient::Ingredient,
        queries::ingredients::get_all::{get_all_ingredients, GetAllIngredientsError},
    },
};

impl IntoResponse for GetAllIngredientsError {
    fn into_response(self) -> axum::response::Response {
        let error_type: &str = self.as_ref();
        (
            StatusCode::BAD_REQUEST,
            axum::Json(common::error::ErrorMessage::new(
                error_type,
                self.to_string(),
            )),
        )
            .into_response()
    }
}

impl From<&Ingredient> for IngredientDTO {
    fn from(value: &Ingredient) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
            description: value.description.to_string(),
        }
    }
}

#[tracing::instrument(skip(ingredient_repository))]
pub async fn get_all_ingredients_route(
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
) -> axum::response::Result<Json<Vec<IngredientDTO>>> {
    let result = get_all_ingredients(ingredient_repository).await?;

    Ok(Json(result.iter().map(IngredientDTO::from).collect()))
}
