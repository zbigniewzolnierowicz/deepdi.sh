use backend::{api::AppBuilder, configuration::Settings};
use color_eyre::Result;
use sqlx::PgPool;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout as stdout;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let provider = TracerProvider::builder()
        .with_simple_exporter(stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("recipes");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer::<Registry>().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(telemetry)
        .init();

    let config = Settings::get()?;
    let db = PgPool::connect_lazy_with(config.database.with_db());
    let app = AppBuilder::new().with_postgres_database(db).build()?;
    let listener = config.application.get_listener().await?;
    app.serve(listener).await?;

    Ok(())
}
