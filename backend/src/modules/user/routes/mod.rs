mod login;
mod logout;
mod signup;

use actix_web::{web, Scope};

pub fn router(base_route: &str) -> Scope {
    web::scope(base_route).route("signup", web::post().to(signup::create_account))
}
