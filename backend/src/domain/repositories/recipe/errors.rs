use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::recipe::errors::ValidationError;

// TODO: split into separate get, update, insert and delete errors that get combined in
// RecipeRepositoryError

#[derive(Error, Debug)]
pub enum RecipeRepositoryError {
    #[error("The recipe with ID of {0} was not found")]
    NotFound(Uuid),
    #[error("The recipe with field {0} of the given value already exists")]
    Conflict(String),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}
