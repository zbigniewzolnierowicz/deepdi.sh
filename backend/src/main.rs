use backend::telemetry;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("recipes".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let server = backend::run(std::net::TcpListener::bind(("0.0.0.0", 8111))?)?;
    server.await
}
