use actix_web::Responder;
use tracing::instrument;

#[instrument]
pub async fn create_account() -> impl Responder {
    "SIGNUP"
}
