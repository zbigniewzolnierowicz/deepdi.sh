use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fmt::Display;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
#[aliases(ErrorMessageWithJsonValue = ErrorMessage<Value>)]
pub struct ErrorMessage<T: Display> {
    pub timestamp: DateTime<Utc>,
    pub error: T,
}

impl<T: Display> ErrorMessage<T> {
    pub fn new(error: T) -> Self {
        Self {
            timestamp: Utc::now(),
            error,
        }
    }
}
