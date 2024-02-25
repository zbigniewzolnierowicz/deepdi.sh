use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::ingredients::{errors::CreateIngredientError, models::Ingredient};

#[utoipa::path(
    post,
    path = "/ingredients/create",
    request_body = CreateIngredientDTO,
    responses(
        (status = 201, description = "Recipe was created", body = IngredientDTO),
        (status = 400, description = "Ingredients are missing", body = ErrorMessageWithJsonValue),
        (status = 500, description = "Fatal error", body = ErrorMessageWithJsonValue)
    )
)]
#[instrument(name = "Create ingredient", skip(db))]
pub async fn create_ingredient(
    db: web::Data<PgPool>,
    body: web::Json<common::CreateIngredientDTO>,
) -> Result<HttpResponse, CreateIngredientError> {
    let result = insert_ingredient_into_db(&db, &body).await?;
    let result: common::IngredientDTO = result.into();

    Ok(HttpResponse::Created().json(result))
}

#[instrument(name = "Persist ingredients in database", skip(db))]
async fn insert_ingredient_into_db(
    db: &PgPool,
    body: &common::CreateIngredientDTO,
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
