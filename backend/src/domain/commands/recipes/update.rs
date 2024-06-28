use std::collections::BTreeMap;

use common::ServingsTypeDTO;
use uuid::Uuid;

use crate::domain::entities::recipe::errors::ValidationError;
use crate::domain::entities::recipe::RecipeChangeset;
use crate::domain::repositories::recipe::errors::{
    GetRecipeByIdError, UpdateRecipeError as UpdateRecipeErrorInternal,
};
use crate::domain::repositories::recipe::RecipeRepositoryService;

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum UpdateRecipeError {
    #[error("Could not find the ingredient with ID {0}")]
    NotFound(Uuid),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

impl From<UpdateRecipeErrorInternal> for UpdateRecipeError {
    fn from(value: UpdateRecipeErrorInternal) -> Self {
        match value {
            UpdateRecipeErrorInternal::Get(GetRecipeByIdError::NotFound(id)) => Self::NotFound(id),
            err => Self::Unknown(err.into()),
        }
    }
}

#[derive(Default)]
pub struct UpdateRecipe {
    pub name: Option<String>,
    pub description: Option<String>,
    pub steps: Option<Vec<String>>,
    pub time: Option<BTreeMap<String, std::time::Duration>>,
    pub servings: Option<ServingsTypeDTO>,
}

impl TryFrom<UpdateRecipe> for RecipeChangeset {
    type Error = ValidationError;
    fn try_from(value: UpdateRecipe) -> Result<Self, Self::Error> {
        Ok(Self {
            steps: match value.steps.clone() {
                Some(r) => Some(r.try_into()?),
                None => None,
            },
            time: value.time,
            name: value.name,
            servings: value.servings.map(|s| s.into()),
            description: value.description,
        })
    }
}

pub async fn update_recipe(
    recipe_repo: RecipeRepositoryService,
    input: &Uuid,
    update: UpdateRecipe,
) -> Result<(), UpdateRecipeError> {
    let changeset = update.try_into()?;
    recipe_repo
        .update(input, changeset)
        .await
        .map_err(UpdateRecipeError::from)?;

    Ok(())
}
