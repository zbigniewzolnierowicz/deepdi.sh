use actix_web::HttpResponse;

use crate::modules::recipes::models::{Ingredient, Recipe};

pub async fn get_recipe() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
