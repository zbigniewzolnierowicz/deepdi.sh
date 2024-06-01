use uuid::Uuid;

use crate::domain::entities::ingredient::*;
use crate::domain::repositories::ingredients::{
    errors::InsertIngredientError, IngredientRepositoryService,
};

use self::errors::ValidationError;
use self::types::DietFriendly;

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum CreateIngredientError {
    #[error("The field {0} was empty")]
    EmptyField(&'static str),
    #[error(
        "A conflict has occured - an ingredient with field {0} of the given value already exists."
    )]
    Conflict(String),
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<InsertIngredientError> for CreateIngredientError {
    fn from(value: InsertIngredientError) -> Self {
        match value {
            InsertIngredientError::Conflict(field) => Self::Conflict(field),
            e => Self::Internal(e.into()),
        }
    }
}

impl From<ValidationError> for CreateIngredientError {
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::EmptyField(field) => Self::EmptyField(field[0]),
            e => Self::Internal(e.into()),
        }
    }
}

#[derive(Debug)]
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
                .collect::<Vec<_>>()
                .into(),
        })
    }
}

#[tracing::instrument("[COMMAND] Creating a new ingredient", skip(repo))]
pub async fn create_ingredient(
    repo: IngredientRepositoryService,
    input: &CreateIngredient<'_>,
) -> Result<Ingredient, CreateIngredientError> {
    let ingredient = Ingredient::try_from(input)?;
    let ingredient = repo.insert(ingredient).await?;
    Ok(ingredient)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::repositories::ingredients::InMemoryIngredientRepository;

    use super::*;

    #[tokio::test]
    async fn creates_an_ingredient() {
        let given = CreateIngredient {
            name: "Tomato",
            description: "Description of a tomato",
            diet_friendly: vec!["Vegan".into()],
        };
        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));

        let when = create_ingredient(repo.clone(), &given).await.unwrap();

        // THEN

        assert_eq!(when.name.as_ref(), "Tomato");
        assert_eq!(when.description.as_ref(), "Description of a tomato");
        assert!(when.diet_friendly.contains(&DietFriendly::Vegan));
    }

    #[tokio::test]
    async fn incorrect_diets_do_not_get_included() {
        let given = CreateIngredient {
            name: "Tomato",
            description: "Description of a tomato",
            diet_friendly: vec!["Vegan".into(), "INVALID DIET".into()],
        };

        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));

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

        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));

        let when = create_ingredient(repo.clone(), &given).await.unwrap_err();

        // THEN

        assert!(matches!(when, CreateIngredientError::EmptyField("name")));
    }

    #[tokio::test]
    async fn empty_description_fails() {
        let given = CreateIngredient {
            name: "Tomato",
            description: "",
            diet_friendly: vec![],
        };

        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));

        let when = create_ingredient(repo.clone(), &given).await.unwrap_err();

        // THEN

        assert!(matches!(
            when,
            CreateIngredientError::EmptyField("description")
        ));
    }

    #[tokio::test]
    async fn incorrect_ingredient_is_not_persisted() {
        let given = CreateIngredient {
            name: "",
            description: "Description of a tomato",
            diet_friendly: vec![],
        };

        let repo: IngredientRepositoryService =
            Arc::new(Box::new(InMemoryIngredientRepository::new()));

        let when = create_ingredient(repo.clone(), &given).await.unwrap_err();

        // THEN

        assert!(matches!(when, CreateIngredientError::EmptyField(_)));

        assert!(!&repo
            .get_all()
            .await
            .unwrap()
            .into_iter()
            .any(|x| x.name.as_str() == given.name))
    }
}
