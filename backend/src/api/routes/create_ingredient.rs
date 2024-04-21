use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::{CreateIngredientDTO, IngredientDTO};

use crate::{
    api::AppState,
    domain::commands::ingredients::create::{
        create_ingredient, CreateIngredient, CreateIngredientError,
    },
};

impl IntoResponse for CreateIngredientError {
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

#[tracing::instrument(skip(ingredient_repository))]
pub async fn create_ingredient_route(
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
    Json(body): Json<CreateIngredientDTO>,
) -> Result<Json<IngredientDTO>, CreateIngredientError> {
    let input = CreateIngredient {
        name: &body.name,
        description: &body.description,
        diet_friendly: body.diet_friendly.unwrap_or_default(),
    };
    let result = create_ingredient(ingredient_repository, &input).await?;

    Ok(Json(IngredientDTO {
        id: result.id,
        name: result.name.to_string(),
        description: result.description.to_string(),
    }))
}
