use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

use crate::modules::user::middleware::LoginStatus;

#[derive(Debug, thiserror::Error)]
pub enum LoginStatusError {
    #[error("User is already logged in")]
    AlreadyLoggedIn,

    #[error("User is not logged in")]
    NotLoggedIn,

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Report),
}

impl ResponseError for LoginStatusError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::AlreadyLoggedIn => StatusCode::BAD_REQUEST,
            Self::NotLoggedIn => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorMessage::new(self.to_string()))
    }
}

impl From<LoginStatus> for LoginStatusError {
    fn from(value: LoginStatus) -> Self {
        match value {
            LoginStatus::LoggedIn => Self::AlreadyLoggedIn,
            LoginStatus::LoggedOut => Self::NotLoggedIn,
        }
    }
}
