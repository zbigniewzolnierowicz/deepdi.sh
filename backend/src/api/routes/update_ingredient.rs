use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use common::{IngredientDTO, UpdateIngredientDTO};
use uuid::Uuid;

use crate::{
    api::AppState,
    domain::commands::ingredients::update::{
        update_ingredient, UpdateIngredient, UpdateIngredientError,
    },
};

impl From<UpdateIngredientDTO> for UpdateIngredient {
    fn from(value: UpdateIngredientDTO) -> Self {
        Self {
            name: value.name,
            description: value.description,
            diet_friendly: value.diet_friendly,
        }
    }
}

impl IntoResponse for UpdateIngredientError {
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

#[tracing::instrument("[ROUTE] Updating an existing ingredient", skip(ingredient_repository))]
pub async fn update_ingredient_route(
    Path(ingredient_id): Path<Uuid>,
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
    Json(body): Json<UpdateIngredientDTO>,
) -> Result<Json<IngredientDTO>, UpdateIngredientError> {
    let result = update_ingredient(ingredient_repository, ingredient_id, &body.into()).await?;

    Ok(Json(result.into()))
}
