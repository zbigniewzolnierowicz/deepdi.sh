use eyre::eyre;
use sqlx::Error as SQLXError;
use std::sync::PoisonError;
use thiserror::Error;

use crate::domain::repositories::recipe::errors::constraint_to_field;

#[derive(Error, Debug)]
pub enum AddIngredientIntoRecipeError {
    #[error("The recipe with field {0} of the given value already exists")]
    Conflict(String),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for AddIngredientIntoRecipeError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Recipe repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

impl From<SQLXError> for AddIngredientIntoRecipeError {
    fn from(e: SQLXError) -> Self {
        match e {
            SQLXError::Database(dberror) => Self::Conflict(
                constraint_to_field(dberror.constraint().unwrap_or_default()).to_string(),
            ),
            e => Self::UnknownError(e.into()),
        }
    }
}
