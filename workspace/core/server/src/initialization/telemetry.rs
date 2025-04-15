use async_graphql::Error;
use log::info;
use opentelemetry::propagation::TextMapCompositePropagator;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::metrics::{
    MeterProviderBuilder, PeriodicReader, SdkMeterProvider, Temporality,
};
use opentelemetry_sdk::propagation::{BaggagePropagator, TraceContextPropagator};
use opentelemetry_sdk::trace::{RandomIdGenerator, SdkTracerProvider};
use opentelemetry_sdk::Resource;
use std::collections::HashMap;
use std::env;
use tracing::Level;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn new_tracing_provider(resource: Resource) -> Result<SdkTracerProvider, Error> {
    let mut provider_builder = SdkTracerProvider::builder().with_resource(resource);
    if let Ok(endpoint) = env::var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT") {
        info!(target: "bosca", "sending traces to: {}", endpoint);
        let mut headers = HashMap::<String, String>::new();
        if let Ok(api_key) = env::var("OTLP_API_KEY") {
            headers.insert("x-api-key".to_string(), api_key);
        }
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
            .with_endpoint(endpoint)
            .with_headers(headers)
            .build()?;
        provider_builder = provider_builder
            .with_id_generator(RandomIdGenerator::default())
            .with_batch_exporter(exporter);
    } else {
        info!(target: "bosca", "no exporter configured");
    }
    Ok(provider_builder.build())
}

fn new_meter_provider(resource: Resource) -> Result<SdkMeterProvider, Error> {
    let mut exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
        .with_temporality(Temporality::default());
    if let Ok(endpoint) = env::var("OTEL_EXPORTER_OTLP_METRICS_ENDPOINT") {
        info!(target: "bosca", "sending metrics to: {}", endpoint);
        exporter = exporter.with_endpoint(endpoint);
        if let Ok(api_key) = env::var("OTLP_API_KEY") {
            let mut headers = HashMap::<String, String>::new();
            headers.insert("x-api-key".to_string(), api_key);
            exporter = exporter.with_headers(headers);
        }
    }
    let reader = PeriodicReader::builder(exporter.build()?)
        .with_interval(std::time::Duration::from_secs(30))
        .build();
    Ok(MeterProviderBuilder::default()
        .with_resource(resource.clone())
        .with_reader(reader)
        .build())
}

pub fn new_tracing() -> Result<TracingConfig, Error> {
    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::default()),
        Box::new(BaggagePropagator::default()),
    ]));

    let resource = Resource::builder().with_attributes(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        "bosca-server",
    )]).build();

    let tracer_provider = new_tracing_provider(resource.clone())?;
    let meter_provider = new_meter_provider(resource)?;

    global::set_tracer_provider(tracer_provider.clone());
    global::set_meter_provider(meter_provider.clone());

    let tracer = tracer_provider.tracer("Bosca");
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(Level::INFO))
        .with(tracing_subscriber::fmt::layer())
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer.clone()))
        .init();


    Ok(TracingConfig {
        tracer_provider,
        meter_provider,
    })
}

pub struct TracingConfig {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
}

impl TracingConfig {
    pub fn shutdown(&self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}
