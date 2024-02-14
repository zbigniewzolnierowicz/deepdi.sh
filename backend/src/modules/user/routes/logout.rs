use crate::modules::user::LogoutError;
use actix_session::Session;
use actix_web::HttpResponse;

pub async fn log_out(session: Session) -> Result<HttpResponse, LogoutError> {
    session.purge();

    Ok(HttpResponse::Ok().body(""))
}
