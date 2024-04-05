use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::IngredientDTO;
use uuid::Uuid;

use crate::{
    api::AppState,
    domain::queries::ingredients::get_by_id::{get_ingredient_by_id, GetIngredientError},
};

impl IntoResponse for GetIngredientError {
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

pub async fn get_ingredient_by_id_route(
    Path(ingredient_id): Path<Uuid>,
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
) -> axum::response::Result<Json<IngredientDTO>> {
    let result = get_ingredient_by_id(ingredient_repository, ingredient_id).await?;

    Ok(Json(IngredientDTO {
        id: result.id,
        name: result.name.to_string(),
        description: result.description.to_string(),
    }))
}
