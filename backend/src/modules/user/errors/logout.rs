use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

use crate::modules::user::domain::SessionValidationError;

#[derive(Debug, thiserror::Error)]
pub enum LogoutError {
    #[error("User has not logged in")]
    NotLoggedIn,

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Report),
}

impl ResponseError for LogoutError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotLoggedIn => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorMessage::new(self.to_string()))
    }
}

impl From<SessionValidationError> for LogoutError {
    fn from(value: SessionValidationError) -> Self {
        match value {
            SessionValidationError::NotLoggedIn => Self::NotLoggedIn,
            SessionValidationError::UnexpectedError(e) => Self::UnexpectedError(e),
        }
    }
}
