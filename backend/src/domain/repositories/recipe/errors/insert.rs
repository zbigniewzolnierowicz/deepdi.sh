use eyre::eyre;
use sqlx::Error as SQLXError;
use std::sync::PoisonError;
use thiserror::Error;

use crate::domain::entities::recipe::errors::ValidationError;

use super::{constraint_to_field, AddIngredientIntoRecipeError};

#[derive(Error, Debug)]
pub enum InsertRecipeError {
    #[error("The recipe with field {0} of the given value already exists")]
    Conflict(String),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    InsertIngredient(#[from] AddIngredientIntoRecipeError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl From<SQLXError> for InsertRecipeError {
    fn from(e: SQLXError) -> Self {
        match e {
            SQLXError::Database(dberror) => Self::Conflict(
                constraint_to_field(dberror.constraint().unwrap_or_default()).to_string(),
            ),
            e => Self::UnknownError(e.into()),
        }
    }
}

impl<T> From<PoisonError<T>> for InsertRecipeError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Recipe repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}
