use actix_web::{web, HttpResponse, Responder, Scope};
use sqlx::PgPool;

#[tracing::instrument]
async fn health_check(db: web::Data<PgPool>) -> impl Responder {
    if db_status(&db).await {
        HttpResponse::Ok()
    } else {
        HttpResponse::InternalServerError()
    }
}

#[tracing::instrument(name = "Check status of database")]
async fn db_status(db: &PgPool) -> bool {
    sqlx::query("SELECT 1;").execute(db).await.map_err(|e| e.to_string()).is_ok()
}

pub fn router(base_route: &str) -> Scope {
    web::scope(base_route).route("/", web::get().to(health_check))
}
