use crate::modules::recipes::models::{Ingredient, RecipeBase, Step};
use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use common::error::ErrorMessage;
use sqlx::PgPool;
use tracing::instrument;

#[derive(Debug, thiserror::Error)]
pub enum RecipeGetError {
    #[error("Recipe does not exist")]
    MissingRecipe,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for RecipeGetError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingRecipe => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorMessage::new(self.to_string()))
    }
}

#[instrument(name = "Getting a recipe", skip(db))]
pub async fn get_recipe(path: web::Path<i32>, db: web::Data<PgPool>) -> Result<HttpResponse, RecipeGetError> {
    let tx = db.begin().await.context("Error making a transaction")?;
    let recipe = get_base_recipe(&db, *path)
        .await?
        .ok_or_else(|| RecipeGetError::MissingRecipe)?;
    let steps = get_steps_for_recipe(&db, &recipe).await?;
    let ingredients = get_ingredients_for_recipe(&db, &recipe).await?;
    tx.commit().await.context("Error committing transaction")?;
    let rec = recipe.into_dto(steps, ingredients);

    Ok(HttpResponse::Ok().json(rec))
}

#[instrument(name = "Getting recipe metadata", skip(db))]
async fn get_base_recipe(db: &PgPool, id: i32) -> anyhow::Result<Option<RecipeBase>> {
    sqlx::query_as!(
        RecipeBase,
        "SELECT id, name, description, user_id FROM recipes WHERE id = $1",
        1
    )
    .fetch_optional(db)
    .await
    .context("Error fetching recipes")
}

#[instrument(name = "Getting steps for recipe", skip(db))]
async fn get_steps_for_recipe(db: &PgPool, recipe: &RecipeBase) -> anyhow::Result<Vec<Step>> {
    let mut steps = sqlx::query_as!(
        Step,
        "SELECT index, instructions FROM steps WHERE steps.recipe_id = $1",
        recipe.id
    )
    .fetch_all(db)
    .await
    .context("Error fetching steps for the recipe")?;

    steps.sort_by_key(|s| s.index);

    Ok(steps)
}

#[instrument(name = "Getting ingredients for recipe", skip(db))]
async fn get_ingredients_for_recipe(
    db: &PgPool,
    recipe: &RecipeBase,
) -> anyhow::Result<Vec<Ingredient>> {
    sqlx::query_as!(
        Ingredient,
        r#"SELECT
               unit, amount, name, ingredient_id
               FROM ingredients_in_recipes
               INNER JOIN ingredients
               ON ingredients.id = ingredients_in_recipes.ingredient_id
               WHERE ingredients_in_recipes.recipe_id = $1;"#,
        recipe.id
    )
    .fetch_all(db)
    .await
    .context("Error fetching ingredients")
}
