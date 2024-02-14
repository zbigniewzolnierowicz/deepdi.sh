use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Wrong password")]
    WrongPassword,

    #[error("Could not find person")]
    NotFound,

    #[error("{0}")]
    ValidationError(String),

    #[error("User is already logged in")]
    AlreadyLoggedIn,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::WrongPassword => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::AlreadyLoggedIn => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorMessage::new(self.to_string()))
    }
}
