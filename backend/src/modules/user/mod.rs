use actix_web::{web, Scope};

pub mod routes;

pub fn router() -> Scope {
    web::scope("")
        .route("signup", web::get().to(routes::create_account))
}
