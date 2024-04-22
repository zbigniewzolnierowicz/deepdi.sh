use std::time::Duration;

use opentelemetry::KeyValue;
use opentelemetry_sdk::{
    resource::{EnvResourceDetector, SdkProvidedResourceDetector, TelemetryResourceDetector},
    runtime, Resource,
};
use tracing_subscriber::{Registry, prelude::__tracing_subscriber_SubscriberExt};

fn init_otel_resource() -> Resource {
    let otlp_resource_detected = Resource::from_detectors(
        Duration::from_secs(3),
        vec![
            Box::new(SdkProvidedResourceDetector),
            Box::new(EnvResourceDetector::new()),
            Box::new(TelemetryResourceDetector),
        ],
    );
    let otlp_resource_override = Resource::new(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        "backend",
    )]);
    otlp_resource_detected.merge(&otlp_resource_override)
}
pub fn init_tracing() -> color_eyre::Result<()> {
    let metrics_pipeline = opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(init_otel_resource())
        .build()?;

    let metrics_subscriber_layer = tracing_opentelemetry::MetricsLayer::new(metrics_pipeline);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(opentelemetry_sdk::trace::config().with_resource(init_otel_resource()))
        .install_batch(runtime::Tokio)?;
    let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let telemetry_subscriber = Registry::default()
        .with(metrics_subscriber_layer)
        .with(tracing_layer);

    tracing::subscriber::set_global_default(telemetry_subscriber)?;
    Ok(())
}
