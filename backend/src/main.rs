use backend::telemetry;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("recipes".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config =
        backend::configuration::Settings::get().expect("Could not read configuration file");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let server = backend::run(
        std::net::TcpListener::bind((config.application.host, config.application.port))?,
        connection_pool,
    )?;
    server.await
}
