use std::time::Duration;

use opentelemetry::{propagation::TextMapCompositePropagator, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    metrics::reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
    propagation::{BaggagePropagator, TraceContextPropagator},
    resource::{EnvResourceDetector, SdkProvidedResourceDetector, TelemetryResourceDetector},
    runtime,
    trace::{Sampler, Tracer},
    Resource,
};
use tracing::Subscriber;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{layer::SubscriberExt, registry::LookupSpan, EnvFilter, Layer};

fn build_resource() -> Resource {
    Resource::from_detectors(
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
    ]))
}

fn build_metrics() -> color_eyre::Result<opentelemetry_sdk::metrics::SdkMeterProvider> {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("grpc://localhost:4317");

    Ok(opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(exporter)
        .with_period(std::time::Duration::from_secs(3))
        .with_resource(build_resource())
        .with_aggregation_selector(DefaultAggregationSelector::new())
        .with_temporality_selector(DefaultTemporalitySelector::new())
        .build()?)
}

pub fn build_otel_layer<S>() -> color_eyre::Result<OpenTelemetryLayer<S, Tracer>>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let otel_rsrc = build_resource();

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
                        .with_resource(otel_rsrc.clone())
                        .with_sampler(Sampler::AlwaysOn),
                )
                .install_batch(opentelemetry_sdk::runtime::Tokio)?,
        ))
}
pub fn build_loglevel_filter_layer() -> tracing_subscriber::filter::EnvFilter {
    // filter what is output on log (fmt)
    // std::env::set_var("RUST_LOG", "warn,otel::tracing=info,otel=debug");
    std::env::set_var(
        "RUST_LOG",
        format!(
            // `otel::tracing` should be a level info to emit opentelemetry trace & span
            // `otel::setup` set to debug to log detected resources, configuration read and infered
            "{},otel::tracing=trace,otel=debug,h2=info",
            std::env::var("RUST_LOG")
                .or_else(|_| std::env::var("OTEL_LOG_LEVEL"))
                .unwrap_or_else(|_| "info".to_string())
        ),
    );
    EnvFilter::from_default_env()
}

pub fn build_logger_text<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    Box::new(tracing_logfmt_otel::layer())
}

pub fn init_tracing() -> color_eyre::Result<()> {
    let subscriber = tracing_subscriber::registry()
        .with(build_otel_layer()?)
        .with(MetricsLayer::new(build_metrics()?))
        .with(build_loglevel_filter_layer())
        .with(build_logger_text());
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
