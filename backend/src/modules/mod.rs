use actix_web::{web, Scope};

pub mod user;

pub fn router() -> Scope {
    web::scope("/user").service(user::router())
}
