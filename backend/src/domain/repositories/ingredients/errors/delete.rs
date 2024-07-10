use eyre::eyre;
use std::sync::PoisonError;
use thiserror::Error;

use super::GetIngredientByIdError;

#[derive(Error, Debug)]
pub enum DeleteIngredientError {
    #[error(transparent)]
    Get(#[from] GetIngredientByIdError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for DeleteIngredientError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}
