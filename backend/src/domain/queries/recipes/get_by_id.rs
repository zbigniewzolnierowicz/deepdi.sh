use uuid::Uuid;

use crate::domain::{
    entities::recipe::{errors::ValidationError, Recipe},
    repositories::recipe::{
        errors::GetRecipeByIdError as GetRecipeByIdErrorInternal, RecipeRepositoryService,
    },
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum GetRecipeError {
    #[error("Could not found the recipe with the ID {0}")]
    NotFound(Uuid),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

impl From<GetRecipeByIdErrorInternal> for GetRecipeError {
    fn from(value: GetRecipeByIdErrorInternal) -> Self {
        match value {
            GetRecipeByIdErrorInternal::NotFound(id) => GetRecipeError::NotFound(id),
            e => e.into(),
        }
    }
}

pub async fn get_recipe_by_id(
    recipe_repo: RecipeRepositoryService,
    input: &Uuid,
) -> Result<Recipe, GetRecipeError> {
    let result = recipe_repo
        .get_by_id(input)
        .await
        .map_err(GetRecipeError::from)?;

    Ok(result)
}
