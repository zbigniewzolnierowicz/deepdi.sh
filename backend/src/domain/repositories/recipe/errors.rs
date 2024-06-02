use std::{collections::HashMap, sync::OnceLock};

use sqlx::Error as SQLXError;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::recipe::errors::ValidationError;

// TODO: split into separate get, update, insert and delete errors that get combined in
// RecipeRepositoryError

#[derive(Error, Debug)]
pub enum GetRecipeByIdError {
    #[error("The recipe with ID of {0} was not found")]
    NotFound(Uuid),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl GetRecipeByIdError {
    pub fn with_id(id: &Uuid, e: SQLXError) -> Self {
        match e {
            SQLXError::RowNotFound => Self::NotFound(*id),
            _ => Self::UnknownError(e.into()),
        }
    }
}

#[derive(Error, Debug)]
pub enum InsertRecipeError {
    #[error("The recipe with field {0} of the given value already exists")]
    Conflict(String),

    #[error(transparent)]
    Get(#[from] GetRecipeByIdError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

/// Turns out Postgres doesn't return the column name for unique constraints isn't returned.
/// This function maps constraints to fields
fn constraint_to_field(field: &str) -> &str {
    static HASHMAP: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    let m = HASHMAP.get_or_init(|| {
        HashMap::from_iter([
            ("ingredients_name_key", "ingredient name"),
            ("ingredients_pkey", "ingredient id"),
            ("recipes_pkey", "recipe id"),
        ])
    });
    m.get(field).unwrap_or(&field)
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

#[derive(Error, Debug)]
pub enum DeleteRecipeError {
    #[error("The recipe with ID of {0} was not found")]
    NotFound(Uuid),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}
