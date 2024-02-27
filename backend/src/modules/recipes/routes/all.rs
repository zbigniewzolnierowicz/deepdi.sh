use actix_web::{web, HttpResponse};
use eyre::Context;
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::recipes::{errors::get::RecipeGetError, models::RecipeBase};

#[utoipa::path(
    get,
    path = "/recipes/get/",
    responses(
        (status = 200, description = "Recipes were fetched", body = Vec<RecipeDTO>),
        (status = 500, description = "Fatal error", body = ErrorMessageWithJsonValue)
    )
)]
#[instrument(name = "Getting a list of recipes", skip(db))]
pub async fn get_all_recipes(
    db: web::Data<PgPool>,
) -> Result<HttpResponse, RecipeGetError> {
    let recipes = get_all_recipes_from_db(&db).await.wrap_err("Database fetch error")?;

    Ok(HttpResponse::Ok().json(recipes))
}

#[instrument(name = "Get all recipes from db", skip(db))]
pub async fn get_all_recipes_from_db(db: &PgPool) -> Result<Vec<RecipeBase>, sqlx::Error> {
    sqlx::query_as!(RecipeBase, "SELECT id, name, description, user_id FROM recipes;").fetch_all(db).await
}
