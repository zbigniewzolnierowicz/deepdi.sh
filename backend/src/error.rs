use chrono::{DateTime, Utc};
use std::fmt::Display;

#[derive(serde::Serialize)]
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
