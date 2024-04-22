use std::time::Duration;

use opentelemetry::{propagation::TextMapCompositePropagator, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    propagation::{BaggagePropagator, TraceContextPropagator},
    resource::{EnvResourceDetector, SdkProvidedResourceDetector, TelemetryResourceDetector},
    trace::{Sampler, Tracer},
    Resource,
};
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, registry::LookupSpan};

pub fn build_otel_layer<S>() -> color_eyre::Result<OpenTelemetryLayer<S, Tracer>>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let otel_rsrc = Resource::from_detectors(
        Duration::from_secs(3),
        vec![
            Box::new(SdkProvidedResourceDetector),
            Box::new(EnvResourceDetector::new()),
            Box::new(TelemetryResourceDetector),
        ],
    )
    .merge(&mut Resource::new(vec![
        KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "backend",
        ),
        KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_VERSION,
            "0.0.0",
        ),
    ]));

    let composite_propagator = TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]);

    opentelemetry::global::set_text_map_propagator(composite_propagator);
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("grpc://localhost:4317");

    Ok(tracing_opentelemetry::layer()
        .with_error_records_to_exceptions(true)
        .with_tracer(
            opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .with_trace_config(
                    opentelemetry_sdk::trace::config()
                        .with_resource(otel_rsrc)
                        .with_sampler(Sampler::AlwaysOn),
                )
                .install_batch(opentelemetry_sdk::runtime::Tokio)?,
        ))
}

pub fn init_tracing() -> color_eyre::Result<()> {
    let subscriber = tracing_subscriber::registry().with(build_otel_layer()?);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
