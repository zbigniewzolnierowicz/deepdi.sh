use actix_web::{http::StatusCode, ResponseError};

use crate::error::ErrorMessage;

#[derive(Debug, thiserror::Error)]
pub enum HealthCheckError {
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl ResponseError for HealthCheckError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::build(self.status_code()).json(ErrorMessage::new(self.to_string()))
    }
}
