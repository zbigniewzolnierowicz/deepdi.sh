use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

#[derive(Debug, thiserror::Error, strum::AsRefStr)]
pub enum GetIngredientError {
    #[error("Ingredient with ID of {0} does not exist")]
    MissingIngredient(i32),

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Report),
}

impl ResponseError for GetIngredientError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingIngredient(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ErrorMessage::new(self.as_ref(), self.to_string()))
    }
}
