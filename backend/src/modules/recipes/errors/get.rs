use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

#[derive(Debug, thiserror::Error, strum::AsRefStr)]
pub enum RecipeGetError {
    #[error("Recipe does not exist")]
    MissingRecipe,

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Error),
}

impl ResponseError for RecipeGetError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingRecipe => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ErrorMessage::new(self.as_ref(), self.to_string()))
    }
}
