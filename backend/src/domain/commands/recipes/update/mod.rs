use std::collections::BTreeMap;

use common::{ServingsTypeDTO, UpdateRecipeDTO};
use uuid::Uuid;

use crate::domain::entities::recipe::errors::ValidationError;
use crate::domain::entities::recipe::{Recipe, RecipeChangeset};
use crate::domain::repositories::recipe::errors::{
    GetRecipeByIdError, UpdateRecipeError as UpdateRecipeErrorInternal,
};
use crate::domain::repositories::recipe::RecipeRepositoryService;

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum UpdateRecipeError {
    #[error("You did not provide any changes to be made")]
    ChangesetEmpty,

    #[error("Could not find the ingredient with ID {0}")]
    NotFound(Uuid),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

impl From<UpdateRecipeErrorInternal> for UpdateRecipeError {
    fn from(value: UpdateRecipeErrorInternal) -> Self {
        Self::Unknown(value.into())
    }
}

impl From<GetRecipeByIdError> for UpdateRecipeError {
    fn from(value: GetRecipeByIdError) -> Self {
        match value {
            GetRecipeByIdError::NotFound(id) => Self::NotFound(id),
            GetRecipeByIdError::ValidationError(err) => err.into(),
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

impl From<UpdateRecipeDTO> for UpdateRecipe {
    fn from(value: UpdateRecipeDTO) -> Self {
        Self {
            name: value.name,
            time: value.time.map(|times| {
                times
                    .into_iter()
                    .map(|(k, v)| (k, std::time::Duration::from_secs(v)))
                    .collect()
            }),
            description: value.description,
            steps: value.steps,
            servings: value.servings,
        }
    }
}

pub async fn update_recipe(
    recipe_repo: RecipeRepositoryService,
    input: &Uuid,
    update: UpdateRecipe,
) -> Result<Recipe, UpdateRecipeError> {
    let changeset: RecipeChangeset = update.try_into()?;
    if changeset.is_empty() {
        return Err(UpdateRecipeError::ChangesetEmpty);
    };

    let recipe = recipe_repo.get_by_id(input).await?;

    recipe_repo.update(&recipe, changeset).await?;

    let recipe = recipe_repo
        .get_by_id(input)
        .await
        .map_err(UpdateRecipeError::from)?;

    Ok(recipe)
}

#[cfg(test)]
mod tests;
