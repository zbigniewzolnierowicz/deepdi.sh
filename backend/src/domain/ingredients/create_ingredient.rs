use uuid::Uuid;

use crate::domain::entities::ingredient::*;
use crate::domain::repositories::ingredients::IngredientRepository;

#[derive(thiserror::Error, Debug)]
pub enum CreateIngredientError {
    #[error("The provided name was empty")]
    EmptyName,
    #[error("The provided description was empty")]
    EmptyDescription,
    #[error("Wrong diet: {0}")]
    WrongDiet(String),
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl TryFrom<String> for IngredientDescription {
    type Error = CreateIngredientError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(CreateIngredientError::EmptyDescription);
        }
        Ok(Self(value))
    }
}

impl TryFrom<String> for DietFriendly {
    type Error = CreateIngredientError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Vegan" => Ok(Self::Vegan),
            "Vegetarian" => Ok(Self::Vegetarian),
            "GlutenFree" => Ok(Self::GlutenFree),
            _ => Err(CreateIngredientError::WrongDiet(value)),
        }
    }
}

pub struct CreateIngredient {
    name: String,
    description: String,
    diet_friendly: Vec<String>,
}

impl TryFrom<String> for IngredientName {
    type Error = CreateIngredientError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(CreateIngredientError::EmptyName);
        }
        Ok(Self(value))
    }
}

pub async fn create_ingredient(
    repo: &mut dyn IngredientRepository,
    input: CreateIngredient,
) -> Result<Ingredient, CreateIngredientError> {
    let ingredient = Ingredient {
        id: Uuid::now_v7(),
        name: input.name.try_into()?,
        description: input.description.try_into()?,
        diet_friendly: input
            .diet_friendly
            .into_iter()
            .filter_map(|x| DietFriendly::try_from(x).ok())
            .collect(),
    };
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
            name: "Tomato".into(),
            description: "Description of a tomato".into(),
            diet_friendly: vec!["Vegan".into()],
        };
        let mut repo = InMemoryIngredientRepository::new();

        let when = create_ingredient(&mut repo, given).await.unwrap();

        // THEN

        assert!(Uuid::try_from(when.id).is_ok());
        assert_eq!(when.name.as_ref(), "Tomato");
        assert_eq!(when.description.as_ref(), "Description of a tomato");
        assert!(when.diet_friendly.contains(&DietFriendly::Vegan));

        let _result_in_repo = repo
            .0
            .into_iter()
            .find(|x| x.id == when.id)
            .expect("The result is not in the repository");
    }

    #[tokio::test]
    async fn incorrect_diets_do_not_get_included() {
        let given = CreateIngredient {
            name: "Tomato".into(),
            description: "Description of a tomato".into(),
            diet_friendly: vec!["Vegan".into(), "INVALID DIET".into()],
        };

        let mut repo = InMemoryIngredientRepository::new();

        let when = create_ingredient(&mut repo, given).await.unwrap();

        // THEN

        assert!(when.diet_friendly.contains(&DietFriendly::Vegan));
        assert_eq!(when.diet_friendly.len(), 1);
    }

    #[tokio::test]
    async fn empty_name_fails() {
        let given = CreateIngredient {
            name: "".into(),
            description: "Description of a tomato".into(),
            diet_friendly: vec![],
        };

        let mut repo = InMemoryIngredientRepository::new();

        let when = create_ingredient(&mut repo, given).await;

        // THEN

        match when {
            Err(CreateIngredientError::EmptyName) => {}
            _ => unreachable!(),
        }
    }

    #[tokio::test]
    async fn empty_description_fails() {
        let given = CreateIngredient {
            name: "Tomato".into(),
            description: "".into(),
            diet_friendly: vec![],
        };

        let mut repo = InMemoryIngredientRepository::new();

        let when = create_ingredient(&mut repo, given).await;

        // THEN

        match when {
            Err(CreateIngredientError::EmptyDescription) => {}
            _ => unreachable!(),
        }
    }
}
