use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use common::error::ErrorMessage;

#[derive(Debug, thiserror::Error, strum::AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum LoginError {
    #[error("Wrong password")]
    WrongPassword,

    #[error("Could not find person")]
    NotFound,

    #[error(transparent)]
    UnexpectedError(#[from] eyre::Report),
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::WrongPassword => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(ErrorMessage::new(self.as_ref(), self.to_string()))
    }
}
