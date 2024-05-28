use uuid::Uuid;

use crate::domain::{
    entities::recipe::Recipe,
    repositories::recipe::{errors::RecipeRepositoryError, RecipeRepositoryService},
};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum GetRecipeError {
    #[error("Could not found the recipe with the ID {0}")]
    NotFound(Uuid),

    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

impl From<RecipeRepositoryError> for GetRecipeError {
    fn from(value: RecipeRepositoryError) -> Self {
        match value {
            RecipeRepositoryError::NotFound(id) => GetRecipeError::NotFound(id),
            e => GetRecipeError::Unknown(e.into()),
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
