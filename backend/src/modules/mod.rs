use actix_web::{web, Scope};

pub mod health;
pub mod user;

pub fn router() -> Scope {
    web::scope("")
        .service(user::router("user"))
        .service(health::router("health"))
}
