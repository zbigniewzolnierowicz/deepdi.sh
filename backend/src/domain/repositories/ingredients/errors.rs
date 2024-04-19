use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::ingredient::errors::ValidationError;

#[derive(Error, Debug)]
pub enum IngredientRepositoryError {
    #[error("The ingredient with ID of {0} was not found")]
    NotFound(Uuid),
    #[error("The ingredient with field {0} of the given value already exists")]
    Conflict(String),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}
