use crate::domain::entities::ingredient::errors::ValidationError as IngredientValidationError;

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum ValidationError {
    #[error("The fields {0:?} was empty")]
    EmptyField(Vec<&'static str>),

    #[error("Field {0} does not match any of: {1:?}")]
    DoesNotMatch(&'static str, &'static [&'static str]),

    #[error("Failed to deserialize field {0}")]
    DeserializationFailed(&'static str, #[source] serde_json::Error),

    #[error("Failed to compute measurement from the following string: {0}")]
    MeasurementComputation(String),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

impl From<IngredientValidationError> for ValidationError {
    fn from(value: IngredientValidationError) -> Self {
        match value {
            IngredientValidationError::EmptyField(e) => Self::EmptyField(e),
            IngredientValidationError::DoesNotMatch(a, b) => Self::DoesNotMatch(a, b),
            IngredientValidationError::Unknown(e) => Self::Unknown(e),
        }
    }
}
