mod events;
mod events_sink;
mod installation;
mod writers;

use axum::extract::{DefaultBodyLimit, State};
use axum::routing::post;
use axum::{extract, response::{IntoResponse}, routing::get, Router};
use std::str::FromStr;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use log::{info, warn};
use serde_json::json;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use opentelemetry::{global, KeyValue};
use tokio::net::TcpListener;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, TracerProvider};
use tokio::signal::unix::{signal, SignalKind};
use tower_http::timeout::TimeoutLayer;

use mimalloc::MiMalloc;
use crate::events::Events;
use crate::installation::Installation;
use crate::writers::arrow::json::sink::JsonSink;
use crate::writers::arrow::schema::SchemaDefinition;
use crate::writers::files::{find_file, watch_files};
use crate::writers::writer::EventsWriter;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn shutdown_hook(writer: Arc<EventsWriter>) {
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received SIGINT, shutting down");
            writer.stop();
            loop {
                if writer.is_active() {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        },
        _ = terminate.recv() => {
            warn!("Received SIGTERM, shutting down");
            writer.stop();
            loop {
                if writer.is_active() {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        }
    }
}

async fn register() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_str("Content-Type").unwrap(), HeaderValue::from_str("application/json").unwrap());
    (headers, json!(Installation::new()).to_string())
}

async fn events(State(writer): State<Arc<EventsWriter>>, extract::Json(payload): extract::Json<Events>) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut payload = payload;
    let now = Utc::now();
    payload.received = Some(now.timestamp_millis());
    payload.received_micros = Some(now.timestamp_subsec_micros());
    writer.write(payload).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error writing payload".to_owned()))?;
    Ok((StatusCode::OK, "OK".to_owned()))
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    structured_logger::Builder::with_level("info")
        .with_target_writer(
            "*",
            structured_logger::async_json::new_writer(tokio::io::stdout()),
        )
        .init();

    let mut provider_builder = TracerProvider::builder().with_config(
        opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "bosca-analytics",
        )])),
    );

    if let Ok(endpoint) = env::var("OTLP_TRACE_ENDPOINT") {
        info!(target: "bosca", "sending traces to: {}", endpoint);

        let exporter = opentelemetry_otlp::new_exporter()
            .http()
            .with_http_client(reqwest::Client::new())
            .with_endpoint(endpoint)
            .build_span_exporter()
            .unwrap();

        let batch = BatchSpanProcessor::builder(exporter, runtime::Tokio)
            .with_batch_config(
                BatchConfigBuilder::default()
                    .with_max_queue_size(4096)
                    .build()
            )
            .build();

        provider_builder = provider_builder.with_span_processor(batch);
    } else {
        info!(target: "bosca", "no exporter configured");
    }

    let provider = provider_builder.build();
    // TODO
    let _ = provider.tracer("Bosca Analytics");
    let _ = global::set_tracer_provider(provider);

    let schema = Arc::new(SchemaDefinition::new());
    let writer_schema = Arc::clone(&schema);
    let writer = Arc::new(EventsWriter::new(8, 10000, move |index| {
        let filepath = find_file(index)?;
        Ok(Box::new(JsonSink::new(Arc::clone(&writer_schema), &filepath, 1000).unwrap()))
    }).await);

    let watch_writer = Arc::clone(&writer);
    tokio::spawn(async {
       watch_files(watch_writer, schema).await;
    });

    let app = Router::new()
        .route("/register", get(register))
        .route("/events", post(events))
        .layer(DefaultBodyLimit::disable())
        .layer(TimeoutLayer::new(Duration::from_secs(600)))
        .with_state(Arc::clone(&writer));

    info!(target: "bosca", "Listening on http://0.0.0.0:8009");

    axum::serve(TcpListener::bind("0.0.0.0:8009").await.unwrap(), app)
        .with_graceful_shutdown(shutdown_hook(writer))
        .await
        .unwrap();
}
