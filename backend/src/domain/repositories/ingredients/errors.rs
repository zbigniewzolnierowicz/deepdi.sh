use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::ingredient::errors::ValidationError;

#[derive(Error, Debug)]
pub enum IngredientRepositoryError {
    #[error("The ingredient with ID of {0} was not found")]
    NotFound(Uuid),
    #[error("An ingredient with these parameters already exists.")]
    Conflict,
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}
