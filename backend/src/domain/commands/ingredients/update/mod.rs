use common::UpdateIngredientDTO;
use uuid::Uuid;

use crate::domain::{
    entities::ingredient::{errors::ValidationError, Ingredient, IngredientChangeset},
    repositories::ingredients::{
        errors::{GetIngredientByIdError, UpdateIngredientError as UpdateIngredientErrorInternal},
        IngredientRepositoryService,
    },
};

#[derive(Debug, Default)]
pub struct UpdateIngredient {
    pub name: Option<String>,
    pub description: Option<String>,
    pub diet_violations: Option<Vec<String>>,
}

impl From<UpdateIngredientDTO> for UpdateIngredient {
    fn from(value: UpdateIngredientDTO) -> Self {
        Self {
            name: value.name,
            description: value.description,
            diet_violations: value.diet_violations,
        }
    }
}

impl TryFrom<&UpdateIngredient> for IngredientChangeset {
    type Error = ValidationError;
    fn try_from(value: &UpdateIngredient) -> Result<Self, Self::Error> {
        let name = match &value.name {
            Some(x) => Some(x.clone().try_into()?),
            None => None,
        };

        let description = match &value.description {
            Some(x) => Some(x.clone().try_into()?),
            None => None,
        };

        let diet_violations = value.diet_violations.as_ref().map(|x| x.clone().into());

        Ok(Self {
            name,
            description,
            diet_violations,
        })
    }
}

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum UpdateIngredientError {
    #[error("Could not find the ingredient with ID {0}")]
    NotFound(Uuid),

    #[error(transparent)]
    ValidationError(ValidationError),

    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<ValidationError> for UpdateIngredientError {
    fn from(value: ValidationError) -> Self {
        Self::ValidationError(value)
    }
}

impl From<UpdateIngredientErrorInternal> for UpdateIngredientError {
    fn from(value: UpdateIngredientErrorInternal) -> Self {
        match value {
            UpdateIngredientErrorInternal::ValidationError(v) => Self::ValidationError(v),
            e => e.into(),
        }
    }
}

impl From<GetIngredientByIdError> for UpdateIngredientError {
    fn from(value: GetIngredientByIdError) -> Self {
        match value {
            GetIngredientByIdError::NotFound(id) => Self::NotFound(id),
            e => e.into(),
        }
    }
}

#[tracing::instrument("[COMMAND] Updating an existing ingredient", skip(repo))]
pub async fn update_ingredient(
    repo: IngredientRepositoryService,
    id: Uuid,
    input: &UpdateIngredient,
) -> Result<Ingredient, UpdateIngredientError> {
    let ingredient_to_change = repo.get_by_id(&id).await?;

    tracing::info!("Serializing input into a changeset");
    let ingredient: IngredientChangeset = input.try_into()?;

    tracing::info!("Sending changeset to ingredient repository");
    repo.update(&ingredient_to_change, ingredient.clone())
        .await?;

    let result = repo.get_by_id(&id).await?;

    Ok(result)
}

#[cfg(test)]
mod tests;
