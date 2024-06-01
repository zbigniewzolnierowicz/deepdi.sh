use chrono::Utc;
use common::error::ErrorMessage;
use reqwest::StatusCode;
use serde::Serialize;

pub trait MakeError<T: Serialize>: AsRef<str> + ToString {
    fn get_kind(&self) -> String {
        self.as_ref().to_string()
    }
    fn get_status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn get_message(&self) -> T;
    fn get_error_message(&self) -> ErrorMessage<T> {
        ErrorMessage {
            error: self.get_message(),
            kind: self.get_kind(),
            timestamp: Utc::now(),
        }
    }
    fn get_json(&self) -> axum::Json<ErrorMessage<T>> {
        axum::Json(self.get_error_message())
    }
}
