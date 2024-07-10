use eyre::eyre;
use sqlx::Error as SQLXError;
use std::sync::PoisonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeleteRecipeError {
    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for DeleteRecipeError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Recipe repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

impl From<SQLXError> for DeleteRecipeError {
    fn from(e: SQLXError) -> Self {
        Self::UnknownError(e.into())
    }
}
