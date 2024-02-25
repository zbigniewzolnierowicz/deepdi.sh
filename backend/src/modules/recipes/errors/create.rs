use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

#[derive(Debug, thiserror::Error)]
pub enum RecipeCreateError {
    #[error("Ingredient with those IDs do not exist: {0:?}")]
    MissingIngredients(Vec<i32>),

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Report),
}

impl ResponseError for RecipeCreateError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingIngredients(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorMessage::new(self.to_string()))
    }
}
