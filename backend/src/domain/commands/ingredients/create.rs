use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entities::ingredient::*;
use crate::domain::repositories::ingredients::{IngredientRepository, IngredientRepositoryError};

use self::errors::ValidationError;
use self::types::DietFriendly;

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum CreateIngredientError {
    #[error("The field {0} was empty")]
    EmptyField(&'static str),
    #[error("A conflict has occured - an ingredient with field {0} of value {1} already exists.")]
    Conflict(&'static str, String),
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<IngredientRepositoryError> for CreateIngredientError {
    fn from(value: IngredientRepositoryError) -> Self {
        match value {
            IngredientRepositoryError::UnknownError(e) => Self::Internal(e),
            IngredientRepositoryError::Conflict(field, value) => Self::Conflict(field, value),
            _ => unreachable!(),
        }
    }
}

impl From<ValidationError> for CreateIngredientError {
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::EmptyField(field) => Self::EmptyField(field),
            e => Self::Internal(e.into()),
        }
    }
}

pub struct CreateIngredient<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub diet_friendly: Vec<String>,
}

impl<'a> TryFrom<&CreateIngredient<'a>> for Ingredient {
    type Error = ValidationError;
    fn try_from(value: &CreateIngredient<'a>) -> Result<Self, Self::Error> {
        Ok(Ingredient {
            id: Uuid::now_v7(),
            name: value.name.try_into()?,
            description: value.description.try_into()?,
            diet_friendly: value
                .diet_friendly
                .clone()
                .into_iter()
                .filter_map(|x| DietFriendly::try_from(x).ok())
                .collect(),
        })
    }
}

pub async fn create_ingredient(
    repo: Arc<dyn IngredientRepository>,
    input: &CreateIngredient<'_>,
) -> Result<Ingredient, CreateIngredientError> {
    let ingredient = Ingredient::try_from(input)?;
    let ingredient = repo.insert(ingredient).await?;
    Ok(ingredient)
}

#[cfg(test)]
mod test {
    use crate::domain::repositories::ingredients::InMemoryIngredientRepository;

    use super::*;

    #[tokio::test]
    async fn creates_an_ingredient() {
        let given = CreateIngredient {
            name: "Tomato",
            description: "Description of a tomato",
            diet_friendly: vec!["Vegan".into()],
        };
        let repo = Arc::new(InMemoryIngredientRepository::new());

        let when = create_ingredient(repo.clone(), &given).await.unwrap();

        // THEN

        assert_eq!(when.name.as_ref(), "Tomato");
        assert_eq!(when.description.as_ref(), "Description of a tomato");
        assert!(when.diet_friendly.contains(&DietFriendly::Vegan));

        assert!(repo
            .0
            .lock()
            .unwrap()
            .clone()
            .iter()
            .any(|x| x.id == when.id))
    }

    #[tokio::test]
    async fn incorrect_diets_do_not_get_included() {
        let given = CreateIngredient {
            name: "Tomato",
            description: "Description of a tomato",
            diet_friendly: vec!["Vegan".into(), "INVALID DIET".into()],
        };

        let repo = Arc::new(InMemoryIngredientRepository::new());

        let when = create_ingredient(repo.clone(), &given).await.unwrap();

        // THEN

        assert!(when.diet_friendly.contains(&DietFriendly::Vegan));
        assert_eq!(when.diet_friendly.len(), 1);
    }

    #[tokio::test]
    async fn empty_name_fails() {
        let given = CreateIngredient {
            name: "",
            description: "Description of a tomato",
            diet_friendly: vec![],
        };

        let repo = Arc::new(InMemoryIngredientRepository::new());

        let when = create_ingredient(repo.clone(), &given).await;

        // THEN

        match when {
            Err(CreateIngredientError::EmptyField("name")) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn empty_description_fails() {
        let given = CreateIngredient {
            name: "Tomato",
            description: "",
            diet_friendly: vec![],
        };

        let repo = Arc::new(InMemoryIngredientRepository::new());

        let when = create_ingredient(repo.clone(), &given).await;

        // THEN

        match when {
            Err(CreateIngredientError::EmptyField("description")) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn incorrect_ingredient_is_not_persisted() {
        let given = CreateIngredient {
            name: "",
            description: "Description of a tomato",
            diet_friendly: vec![],
        };

        let repo = Arc::new(InMemoryIngredientRepository::new());

        let when = create_ingredient(repo.clone(), &given).await;

        // THEN

        match when {
            Err(CreateIngredientError::EmptyField(_)) => {}
            _ => unreachable!(),
        };

        assert!(!repo
            .clone()
            .0
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .any(|x| x.name.as_str() == given.name))
    }
}
