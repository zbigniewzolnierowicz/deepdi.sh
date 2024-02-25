use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

#[derive(Debug, thiserror::Error)]
pub enum CreateIngredientError {
    #[error("This ingredient already exists")]
    AlreadyExists,

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Report),
}

impl From<sqlx::Error> for CreateIngredientError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(e) => {
                if e.is_unique_violation() {
                    Self::AlreadyExists
                } else {
                    Self::UnexpectedError(e.into())
                }
            }
            e => Self::UnexpectedError(e.into()),
        }
    }
}

impl ResponseError for CreateIngredientError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorMessage::new(self.to_string()))
    }
}
