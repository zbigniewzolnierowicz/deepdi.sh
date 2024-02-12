pub mod modules;
pub mod telemetry;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub fn run(listener: std::net::TcpListener, database: PgPool) -> Result<Server, std::io::Error> {
    let database = web::Data::new(database);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(database.clone())
            .service(modules::router())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
