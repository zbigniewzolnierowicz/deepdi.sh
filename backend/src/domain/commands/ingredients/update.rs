use uuid::Uuid;

use crate::domain::{
    entities::ingredient::{errors::ValidationError, Ingredient, IngredientChangeset},
    repositories::ingredients::{
        base::IngredientRepositoryService, errors::IngredientRepositoryError,
    },
};

#[derive(Debug)]
pub struct UpdateIngredient {
    pub name: Option<String>,
    pub description: Option<String>,
    pub diet_friendly: Option<Vec<String>>,
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

        let diet_friendly = value.diet_friendly.as_ref().map(|x| x.clone().into());

        Ok(Self {
            name,
            description,
            diet_friendly,
        })
    }
}

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum UpdateIngredientError {
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<ValidationError> for UpdateIngredientError {
    fn from(value: ValidationError) -> Self {
        Self::Internal(value.into())
    }
}

impl From<IngredientRepositoryError> for UpdateIngredientError {
    fn from(value: IngredientRepositoryError) -> Self {
        Self::Internal(value.into())
    }
}

#[tracing::instrument("[COMMAND] Updating an existing ingredient", skip(repo))]
pub async fn update_ingredient(
    repo: IngredientRepositoryService,
    id: Uuid,
    input: &UpdateIngredient,
) -> Result<Ingredient, UpdateIngredientError> {
    tracing::info!("Serializing input into a changeset");
    let ingredient: IngredientChangeset = input.try_into()?;
    tracing::info!("Sending changeset to ingredient repository");
    let result = repo.update(id, ingredient).await?;
    Ok(result)
}
