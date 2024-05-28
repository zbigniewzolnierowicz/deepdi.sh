use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use common::RecipeDTO;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::AppState,
    domain::queries::recipes::get_by_id::{get_recipe_by_id, GetRecipeError},
};

impl IntoResponse for GetRecipeError {
    fn into_response(self) -> axum::response::Response {
        let error_type: &str = self.as_ref();
        let status = match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (
            status,
            axum::Json(common::error::ErrorMessage::new(
                error_type,
                self.to_string(),
            )),
        )
            .into_response()
    }
}

#[tracing::instrument("[ROUTE] Creating a new recipe", skip(recipe_repository))]
pub async fn get_recipe_by_id_route(
    State(AppState {
        recipe_repository, ..
    }): State<AppState>,
    Path(recipe_id): Path<Uuid>,
) -> Result<Json<RecipeDTO>, GetRecipeError> {
    let result = get_recipe_by_id(recipe_repository, &recipe_id).await?;

    Ok(axum::Json(result.into()))
}
