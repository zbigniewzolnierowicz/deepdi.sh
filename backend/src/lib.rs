pub mod modules;

use actix_web::{dev::Server, web, App, HttpServer, Responder};
use tracing_actix_web::TracingLogger;

async fn index() -> impl Responder {
    "Hello, world!"
}

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(index))
            .service(modules::router())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
