pub mod configuration;
pub mod modules;
pub mod telemetry;

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: std::net::TcpListener,
    database: PgPool,
    session: RedisSessionStore,
    session_key: Key,
) -> Result<Server, std::io::Error> {
    let addr = listener.local_addr()?;
    tracing::info!("Starting listening on {}:{}", addr.ip(), addr.port());
    let database = web::Data::new(database);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(SessionMiddleware::builder(session.clone(), session_key.clone()).build())
            .app_data(database.clone())
            .service(modules::router())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
