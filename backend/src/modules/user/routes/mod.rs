mod login;
mod logout;
mod signup;

use actix_web::{web, HttpResponse, Responder, Scope};

pub async fn get_user_info(path: web::Path<String>) -> impl Responder {
    path.into_inner()
}

pub fn router(base_route: &str) -> Scope {
    web::scope(base_route)
        .route("/", web::get().to(HttpResponse::Ok))
        .route("/signup", web::post().to(signup::create_account))
        .service(
            web::resource("{user_id}")
                .name("user")
                .route(web::get().to(get_user_info)),
        )
}
