use async_graphql::extensions::OpenTelemetry;
use async_graphql::Error;
use log::info;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::trace::{
    BatchConfigBuilder, BatchSpanProcessor, Tracer, TracerProvider,
};
use opentelemetry_sdk::Resource;
use std::env;
use opentelemetry_sdk::runtime::Tokio;

pub fn new_telemetry() -> Result<OpenTelemetry<Tracer>, Error> {
    let mut provider_builder = TracerProvider::builder().with_resource(
        Resource::new_with_defaults(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "bosca-server",
        )]),
    );

    if let Ok(endpoint) = env::var("OTLP_TRACE_ENDPOINT") {
        info!(target: "bosca", "sending traces to: {}", endpoint);
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_http_client(reqwest::Client::new())
            .with_endpoint(endpoint)
            .build()?;
        let batch = BatchSpanProcessor::builder(exporter, Tokio)
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

    let provider = provider_builder.build();
    let tracer = provider.tracer("Bosca");
    let _ = global::set_tracer_provider(provider);

    Ok(OpenTelemetry::<Tracer>::new(tracer))
}
