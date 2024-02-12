use backend::telemetry;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("recipes".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let connection_string = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://recipes:recipes@localhost:5432/recipes".into());

    let connection_pool = PgPoolOptions::new()
        .connect_lazy(&connection_string)
        .unwrap();

    let server = backend::run(
        std::net::TcpListener::bind(("0.0.0.0", 8111))?,
        connection_pool,
    )?;
    server.await
}
