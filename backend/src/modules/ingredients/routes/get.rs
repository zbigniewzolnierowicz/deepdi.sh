use actix_web::{web, HttpResponse};
use eyre::Context;
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::ingredients::{errors::GetIngredientError, models::Ingredient};

#[instrument(name = "Get recipes", skip(db))]
pub async fn get_recipe(
    path: web::Path<i32>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, GetIngredientError> {
    let ingredient = get_ingredient_from_db(&db, &path)
        .await?
        .ok_or(GetIngredientError::MissingIngredient(0))?;

    Ok(HttpResponse::Ok().json(ingredient))
}

#[instrument(name = "Get ingredient from database", skip(db))]
async fn get_ingredient_from_db(db: &PgPool, id: &i32) -> eyre::Result<Option<Ingredient>> {
    sqlx::query_as!(
        Ingredient,
        "SELECT id, name, description FROM ingredients WHERE id = $1",
        id
    )
    .fetch_optional(db)
    .await
    .wrap_err("Failed to fetch ingredient")
}
