use actix_web::{web, Scope};

pub mod user;

pub fn router() -> Scope {
    web::scope("/").service(user::router("/user"))
}
