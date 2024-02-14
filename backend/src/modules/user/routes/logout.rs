use crate::modules::user::{domain::validate_session, errors::logout::LogoutError};
use actix_session::Session;
use actix_web::HttpResponse;

pub async fn log_out(session: Session) -> Result<HttpResponse, LogoutError> {
    validate_session(&session)?;
    session.purge();

    Ok(HttpResponse::Ok().body(""))
}
