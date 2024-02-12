use actix_web::{web, Scope};

pub mod user;
pub mod health;

pub fn router() -> Scope {
    web::scope("").service(user::router("user"))
        .service(health::router("health"))
}
