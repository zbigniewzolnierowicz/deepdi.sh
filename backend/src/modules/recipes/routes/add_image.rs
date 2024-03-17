use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web, HttpResponse};
use tracing::instrument;
use utoipa::ToSchema;

use crate::modules::recipes::errors::get::RecipeGetError;

#[derive(MultipartForm, ToSchema, Debug)]
pub struct ImageUpload {
    #[schema(value_type = Vec<String>, format = Binary)]
    pub images: Vec<TempFile>,
}

#[utoipa::path(
    post,
    path = "/recipes/{recipeId}/image",
    params(("recipeId" = i32,),),
    request_body(content = ImageUpload, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Images were added to the recipe"),
        (status = 500, description = "Fatal error", body = ErrorMessageWithJsonValue)
    )
)]
#[instrument(name = "Add images to recipe", skip(body))]
pub async fn add_images_to_recipe(
    path: web::Path<i32>,
    body: MultipartForm<ImageUpload>,
) -> Result<HttpResponse, RecipeGetError> {
    let TempFile {
        file, content_type, ..
    } = body.0.images.first().unwrap();
    let data = std::fs::read(file.path()).unwrap();

    Ok(HttpResponse::Ok()
        .content_type(content_type.clone().unwrap())
        .body(data))
}
