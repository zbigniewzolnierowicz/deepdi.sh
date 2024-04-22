use backend::{api::AppBuilder, configuration::Settings};
use color_eyre::Result;
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource;
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // TODO: extract to separate file
    // TODO: add more log points

    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(
            opentelemetry_sdk::trace::config().with_resource(Resource::new(vec![
                KeyValue::new(resource::SERVICE_NAME, "backend"),
                KeyValue::new(resource::SERVICE_VERSION, "0.0.0"),
            ])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    Registry::default()
        .with(tracing_subscriber::fmt::layer().with_target(true).pretty())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(telemetry)
        .init();

    let config = Settings::get()?;
    let db = PgPool::connect_lazy_with(config.database.with_db());
    let app = AppBuilder::new().with_postgres_database(db).build()?;
    let listener = config.application.get_listener().await?;
    app.serve(listener).await?;

    Ok(())
}
