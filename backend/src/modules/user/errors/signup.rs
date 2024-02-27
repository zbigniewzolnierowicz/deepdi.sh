use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

#[derive(Debug, thiserror::Error, strum::AsRefStr)]
pub enum SignupError {
    #[error("User with the following data already exists.")]
    AlreadyExists,

    #[error("{0}")]
    Validation(String),

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Report),
}

impl ResponseError for SignupError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AlreadyExists => StatusCode::CONFLICT,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ErrorMessage::new(self.as_ref(), self.to_string()))
    }
}

impl From<sqlx::Error> for SignupError {
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
