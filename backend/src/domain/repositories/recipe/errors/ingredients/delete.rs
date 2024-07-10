use eyre::eyre;
use sqlx::Error as SQLXError;
use std::sync::PoisonError;
use thiserror::Error;

use crate::domain::entities::recipe::errors::ValidationError;

#[derive(Error, Debug)]
pub enum DeleteIngredientFromRecipeError {
    #[error(transparent)]
    ValidationError(ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for DeleteIngredientFromRecipeError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Recipe repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

impl From<SQLXError> for DeleteIngredientFromRecipeError {
    fn from(e: SQLXError) -> Self {
        Self::UnknownError(e.into())
    }
}
