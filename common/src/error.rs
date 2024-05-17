use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, TS, Debug)]
#[aliases(ErrorMessageWithJsonValue = ErrorMessage<Value>)]
#[ts(export)]
pub struct ErrorMessage<T: Serialize> {
    pub kind: String,
    pub timestamp: DateTime<Utc>,
    pub error: T,
}

impl<T: Serialize> ErrorMessage<T> {
    pub fn new(kind: &str, error: T) -> Self {
        Self {
            kind: kind.to_string(),
            timestamp: Utc::now(),
            error,
        }
    }
}
