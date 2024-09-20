use eyre::eyre;
use std::sync::PoisonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeleteIngredientError {
    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for DeleteIngredientError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

impl From<sqlx::Error> for DeleteIngredientError {
    fn from(e: sqlx::Error) -> Self {
        Self::UnknownError(e.into())
    }
}
