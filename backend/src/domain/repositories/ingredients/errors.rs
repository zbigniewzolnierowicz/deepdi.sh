use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum IngredientRepositoryError {
    #[error("The ingredient with ID of {0} was not found")]
    NotFound(Uuid),
    #[error("The ingredient with field {0} of value {1} already exists")]
    Conflict(&'static str, String),
    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}
