use eyre::eyre;
use std::sync::PoisonError;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::ingredient::errors::ValidationError;

#[derive(Error, Debug)]
pub enum GetIngredientByIdError {
    #[error("The ingredient with ID of {0} was not found")]
    NotFound(Uuid),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for GetIngredientByIdError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

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

#[derive(Error, Debug)]
pub enum InsertIngredientError {
    #[error("The ingredient with field {0} of the given value already exists")]
    Conflict(String),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for InsertIngredientError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

#[derive(Error, Debug)]
pub enum UpdateIngredientError {
    #[error("The ingredient with ID of {0} was not found")]
    NotFound(Uuid),

    #[error("The ingredient with field {0} of the given value already exists")]
    Conflict(String),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for UpdateIngredientError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Ingredient repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}
