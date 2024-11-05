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
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
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
use crate::writers::files::{find_file, watch_files, Config};
use crate::writers::http::sink::HttpSink;
use crate::writers::writer::EventsWriter;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn shutdown_hook(writer: Arc<EventsWriter>, watching: Arc<AtomicBool>) {
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received SIGINT, shutting down");
            writer.stop().await;
            loop {
                if writer.is_active() || watching.load(Relaxed) {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        },
        _ = terminate.recv() => {
            warn!("Received SIGTERM, shutting down");
            writer.stop().await;
            loop {
                if writer.is_active() || watching.load(Relaxed) {
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

async fn index() -> Result<(StatusCode, String), (StatusCode, String)> {
    Ok((StatusCode::OK, "OK".to_owned()))
}

async fn health() -> Result<(StatusCode, String), (StatusCode, String)> {
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

    let forward_url = env::var("FORWARD_URL").unwrap_or("".to_owned());
    let config = if forward_url.is_empty() {
        Some(Config {
            batches_dir: if let Ok(batches_dir) = env::var("BATCHES_DIR") {
                batches_dir
            } else {
                warn!("missing BATCHES_DIR, defaulting to ./analytics/batches");
                "./analytics/batches".to_owned()
            },
            pending_objects_dir: if let Ok(objects_dir) = env::var("PENDING_OBJECTS_DIR") {
                objects_dir
            } else {
                warn!("missing PENDING_OBJECTS_DIR, defaulting to ./analytics/objects");
                "./analytics/objects".to_owned()
            },
            temp_dir: if let Ok(batches_dir) = env::var("TEMP_DIR") {
                batches_dir
            } else {
                warn!("missing TEMP_DIR, defaulting to ./analytics/temp");
                "./analytics/temp".to_owned()
            },
            max_file_size: if let Ok(size) = env::var("MAX_JSON_FILE_SIZE") {
                if let Ok(size) = size.parse() {
                    size
                } else {
                    warn!("invalid MAX_JSON_FILE_SIZE, defaulting to 250MB");
                    262144000
                }
            } else {
                warn!("missing MAX_JSON_FILE_SIZE, defaulting to 250MB");
                262144000
            },
        })
    } else {
        None
    };

    let schema = Arc::new(SchemaDefinition::new());
    let writer_schema = Arc::clone(&schema);
    let writer_config = config.clone();
    // TODO: make these configurable
    let writer = Arc::new(EventsWriter::new(8, 10000, move |index| {
        Ok(if !forward_url.is_empty() {
            Box::new(HttpSink::new(forward_url.clone()))
        } else {
            let filepath = find_file(index, writer_config.as_ref().unwrap().clone())?;
            Box::new(JsonSink::new(Arc::clone(&writer_schema), &filepath, 250).unwrap())
        })
    }).await);

    let watching = Arc::new(AtomicBool::new(false));
    if let Some(config) = config {
        let watch_writer = Arc::clone(&writer);
        let watch_config = config.clone();
        let watch_watching = Arc::clone(&watching);
        tokio::spawn(async {
            watch_files(watch_writer, schema, watch_config, watch_watching).await;
        });
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/register", get(register))
        .route("/events", post(events))
        .layer(DefaultBodyLimit::disable())
        .layer(TimeoutLayer::new(Duration::from_secs(600)))
        .with_state(Arc::clone(&writer));

    info!(target: "bosca", "Listening on http://0.0.0.0:8009");

    axum::serve(TcpListener::bind("0.0.0.0:8009").await.unwrap(), app)
        .with_graceful_shutdown(shutdown_hook(writer, watching))
        .await
        .unwrap();
}
