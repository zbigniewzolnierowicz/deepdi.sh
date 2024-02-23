use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::ingredients::{errors::CreateIngredientError, models::Ingredient};

#[instrument(name = "Create ingredient", skip(db))]
pub async fn create_ingredient(
    db: web::Data<PgPool>,
    body: web::Json<common::CreateIngredient>,
) -> Result<HttpResponse, CreateIngredientError> {
    let result = insert_ingredient_into_db(&db, &body).await?;
    let result: common::Ingredient = result.into();

    Ok(HttpResponse::Created().json(result))
}

async fn insert_ingredient_into_db(
    db: &PgPool,
    body: &common::CreateIngredient,
) -> Result<Ingredient, sqlx::Error> {
    sqlx::query_as!(
        Ingredient,
        "INSERT INTO ingredients (name, description) VALUES ($1, $2) RETURNING id, name, description",
        body.name,
        body.description
    )
    .fetch_one(db)
    .await
}
