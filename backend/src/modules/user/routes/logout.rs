use actix_session::Session;
use actix_web::HttpResponse;

#[utoipa::path(
    post,
    path = "/user/logout",
    responses(
        (status = 200, description = "User was logged out"),
        (status = 500, description = "Fatal error", body = ErrorMessageWithJsonValue)
    )
)]
/// Endpoint for logging out
pub async fn log_out(session: Session) -> HttpResponse {
    session.purge();

    HttpResponse::Ok().finish()
}
