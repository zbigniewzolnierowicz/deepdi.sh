use std::fmt::Display;

use axum::response::IntoResponse;
use reqwest::StatusCode;

use super::MakeError;

#[derive(Debug)]
pub struct ApiError {
    pub kind: &'static str,
    pub status: StatusCode,
    pub message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl MakeError<String> for ApiError {
    fn get_kind(&self) -> String {
        self.kind.to_string()
    }
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

// We implement `IntoResponse` so `ApiError` can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.get_json()).into_response()
    }
}
