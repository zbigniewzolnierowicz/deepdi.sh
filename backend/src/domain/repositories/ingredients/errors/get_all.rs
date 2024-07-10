use eyre::eyre;
use std::sync::PoisonError;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::ingredient::errors::ValidationError;

#[derive(Error, Debug)]
pub enum GetAllIngredientsError {
    #[error("The ingredients with IDs of {0:?} were not found")]
    MultipleIngredientsMissing(Vec<Uuid>),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for GetAllIngredientsError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}
