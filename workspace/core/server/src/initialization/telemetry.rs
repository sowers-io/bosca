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
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, SdkTracer, SdkTracerProvider};
use opentelemetry_sdk::Resource;
use std::collections::HashMap;
use std::env;

fn new_tracing_provider(resource: Resource) -> Result<SdkTracerProvider, Error> {
    let mut provider_builder = SdkTracerProvider::builder().with_resource(resource);
    if let Ok(endpoint) = env::var("OTLP_TRACE_ENDPOINT") {
        info!(target: "bosca", "sending traces to: {}", endpoint);
        let mut headers = HashMap::<String, String>::new();
        if let Ok(api_key) = env::var("OTLP_API_KEY") {
            headers.insert("x-api-key".to_string(), api_key);
        }
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_http_client(reqwest::Client::new())
            .with_endpoint(endpoint)
            .with_headers(headers)
            .build()?;
        let batch = BatchSpanProcessor::builder(exporter)
            .with_batch_config(
                BatchConfigBuilder::default()
                    .with_max_queue_size(4096)
                    .build(),
            )
            .build();
        provider_builder = provider_builder.with_span_processor(batch);
    } else {
        info!(target: "bosca", "no exporter configured");
    }
    Ok(provider_builder.build())
}

fn new_meter_provider(resource: Resource) -> Result<SdkMeterProvider, Error> {
    let mut exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_http_client(reqwest::Client::new())
        .with_temporality(Temporality::default());
    let mut headers = HashMap::<String, String>::new();
    if let Ok(api_key) = env::var("OTLP_API_KEY") {
        headers.insert("x-api-key".to_string(), api_key);
    }
    exporter = exporter.with_headers(headers);
    if let Ok(endpoint) = env::var("OTLP_METER_ENDPOINT") {
        info!(target: "bosca", "sending metrics to: {}", endpoint);
        exporter = exporter.with_endpoint(endpoint);
    }
    let reader = PeriodicReader::builder(exporter.build()?)
        .with_interval(std::time::Duration::from_secs(30))
        .build();
    Ok(MeterProviderBuilder::default()
        .with_resource(resource.clone())
        .with_reader(reader)
        .build())
}

pub fn new_tracer() -> Result<SdkTracer, Error> {
    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::default()),
        Box::new(BaggagePropagator::default()),
    ]));
    let resource = Resource::builder().with_attributes(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        "bosca-server",
    )]).build();
    let trace_provider = new_tracing_provider(resource.clone())?;
    let tracer = trace_provider.tracer("Bosca");
    let _ = global::set_tracer_provider(trace_provider);
    let meter_provider = new_meter_provider(resource)?;
    global::set_meter_provider(meter_provider);
    Ok(tracer)
}
