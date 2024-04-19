use async_trait::async_trait;
use regex::Regex;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::ingredient::{Ingredient, IngredientModel};

use super::{base::IngredientRepository, errors::IngredientRepositoryError};

pub struct PostgresIngredientRepository(pub PgPool, Regex);

#[async_trait]
impl IngredientRepository for PostgresIngredientRepository {
    async fn insert(
        &self,
        ingredient: Ingredient,
    ) -> Result<Ingredient, IngredientRepositoryError> {
        let diet_friendly: Vec<String> = ingredient
            .clone()
            .diet_friendly
            .0
            .into_iter()
            .map(|d| d.to_string())
            .collect();

        let ingredient = sqlx::query_as!(
            IngredientModel,
            r#"
                INSERT INTO ingredients (id, name, description, diet_friendly)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, description, diet_friendly
            "#,
            ingredient.id,
            &ingredient.name,
            &ingredient.description,
            &diet_friendly
        )
        .fetch_one(&self.0)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::Database(dberror) if dberror.is_unique_violation() => {
                let constraint = dberror.constraint().unwrap_or_default().to_string();
                println!("{:?}", self.1.captures(&constraint));

                if let Some(captures) = self.1.captures(&constraint) {
                    let field = captures.name("field");
                    let id = captures.name("pkey");

                    if let Some(field) = field {
                        IngredientRepositoryError::Conflict(field.as_str().to_string())
                    } else if id.is_some() {
                        IngredientRepositoryError::Conflict("id".to_string())
                    } else {
                        IngredientRepositoryError::Conflict(constraint)
                    }
                } else {
                    IngredientRepositoryError::Conflict(constraint)
                }
            }
            _ => IngredientRepositoryError::UnknownError(e.into()),
        })?;

        Ok(ingredient.try_into()?)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Ingredient, IngredientRepositoryError> {
        let ingredient = sqlx::query_as!(
            IngredientModel,
            r#"
                SELECT id, name, description, diet_friendly
                FROM ingredients
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.0)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => IngredientRepositoryError::NotFound(id),
            _ => IngredientRepositoryError::UnknownError(e.into()),
        })?;

        Ok(ingredient.try_into()?)
    }

    async fn get_all(&self) -> Result<Vec<Ingredient>, IngredientRepositoryError> {
        todo!()
    }
}

impl PostgresIngredientRepository {
    pub fn new(pool: PgPool) -> Self {
        let r = Regex::new(r"^(?:ingredients)_(?<field>.*)_(?:key)|(?<pkey>pkey)").unwrap();
        Self(pool, r)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::domain::entities::ingredient::types::WhichDiets;

    use super::*;

    #[sqlx::test]
    async fn insert_ingredient_succeeds(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());

        repo.insert(Ingredient {
            id: Uuid::from_u128(1),
            name: "Ingredient name".try_into().unwrap(),
            description: "Ingredient description".try_into().unwrap(),
            diet_friendly: WhichDiets(vec![]),
        })
        .await
        .unwrap();

        let ingredient = sqlx::query_as!(
            IngredientModel,
            "SELECT id, name, description, diet_friendly FROM ingredients WHERE id = $1",
            Uuid::from_u128(1)
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        assert_eq!(ingredient.len(), 1);
    }

    #[sqlx::test]
    async fn insert_ingredient_that_already_exists_fails_id(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());

        repo.insert(Ingredient {
            id: Uuid::from_u128(1),
            name: "Ingredient name".try_into().unwrap(),
            description: "Ingredient description".try_into().unwrap(),
            diet_friendly: WhichDiets(vec![]),
        })
        .await
        .unwrap();

        let result = repo
            .insert(Ingredient {
                id: Uuid::from_u128(1),
                name: "Ingredient name 2".try_into().unwrap(),
                description: "Ingredient description 2".try_into().unwrap(),
                diet_friendly: WhichDiets(vec![]),
            })
            .await
            .unwrap_err();

        match result {
            IngredientRepositoryError::Conflict(fieldname) => {
                assert_eq!(fieldname, "id");
            }
            _ => unreachable!(),
        };
    }

    #[sqlx::test]
    async fn insert_ingredient_that_already_exists_fails_name(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());

        repo.insert(Ingredient {
            id: Uuid::from_u128(1),
            name: "Ingredient name".try_into().unwrap(),
            description: "Ingredient description".try_into().unwrap(),
            diet_friendly: WhichDiets(vec![]),
        })
        .await
        .unwrap();

        let result = repo
            .insert(Ingredient {
                id: Uuid::from_u128(2),
                name: "Ingredient name".try_into().unwrap(),
                description: "Ingredient description".try_into().unwrap(),
                diet_friendly: WhichDiets(vec![]),
            })
            .await
            .unwrap_err();

        match result {
            IngredientRepositoryError::Conflict(fieldname) => {
                assert_eq!(fieldname, "name");
            }
            _ => unreachable!(),
        };
    }

    #[sqlx::test]
    async fn insert_ingredient_that_already_exists_fails_description(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());

        repo.insert(Ingredient {
            id: Uuid::from_u128(1),
            name: "Ingredient name".try_into().unwrap(),
            description: "Ingredient description".try_into().unwrap(),
            diet_friendly: WhichDiets(vec![]),
        })
        .await
        .unwrap();

        let result = repo
            .insert(Ingredient {
                id: Uuid::from_u128(2),
                name: "Ingredient name 2".try_into().unwrap(),
                description: "Ingredient description".try_into().unwrap(),
                diet_friendly: WhichDiets(vec![]),
            })
            .await
            .unwrap_err();

        match result {
            IngredientRepositoryError::Conflict(fieldname) => {
                assert_eq!(fieldname, "description");
            }
            _ => unreachable!(),
        };
    }

    // TODO: Add tests for get_by_id and get_all
}
