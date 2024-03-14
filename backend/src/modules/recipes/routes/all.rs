use actix_web::{web, HttpResponse};
use eyre::Context;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::modules::recipes::{errors::get::RecipeGetError, models::RecipeBase};

#[derive(Serialize, Deserialize, utoipa::IntoParams, display_json::DebugAsJson)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub count: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/recipes/get/",
    params(Pagination),
    responses(
        (status = 200, description = "Recipes were fetched", body = Vec<RecipeDTO>),
        (status = 500, description = "Fatal error", body = ErrorMessageWithJsonValue)
    )
)]
#[instrument(name = "Getting a list of recipes", skip(db))]
pub async fn get_all_recipes(
    db: web::Data<PgPool>,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, RecipeGetError> {
    let recipes = get_all_recipes_from_db(&db, &pagination.0)
        .await
        .wrap_err("Database fetch error")?;

    Ok(HttpResponse::Ok().json(recipes))
}

#[instrument(name = "Get all recipes from db", skip(db))]
pub async fn get_all_recipes_from_db(
    db: &PgPool,
    pagination: &Pagination,
) -> Result<Vec<RecipeBase>, sqlx::Error> {
    sqlx::query_as!(
        RecipeBase,
        "SELECT id, name, description, user_id FROM recipes ORDER BY id ASC LIMIT $1 OFFSET $2;",
        pagination.count.unwrap_or(10),
        pagination.offset.unwrap_or(0)
    )
    .fetch_all(db)
    .await
}
