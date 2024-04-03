use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::IngredientDTO;

use crate::{
    api::AppState,
    domain::queries::get_all_ingredients::{get_all_ingredients, GetAllIngredientsError},
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

pub async fn get_all_ingredients_route(
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
) -> axum::response::Result<Json<Vec<IngredientDTO>>> {
    let result = get_all_ingredients(ingredient_repository).await?;

    Ok(Json(
        result
            .iter()
            .map(|ing| IngredientDTO {
                id: ing.id,
                name: ing.name.to_string(),
                description: ing.description.to_string(),
            })
            .collect(),
    ))
}
