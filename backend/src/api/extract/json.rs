use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    response::IntoResponse,
};
use serde::Serialize;

use crate::api::errors::api::ApiError;

// create an extractor that internally uses `axum::Json` but has a custom rejection
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

// We implement `IntoResponse` for our extractor so it can be used as a response
impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

// We implement `From<JsonRejection> for ApiError`
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            kind: "JSON_PARSE",
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}
