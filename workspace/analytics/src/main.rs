mod events;
mod events_sync;
mod installation;
mod writers;

use axum::extract::{DefaultBodyLimit, State};
use axum::routing::post;
use axum::{extract, response::{self, IntoResponse}, routing::get, Router};
use std::str::FromStr;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use log::{error, info, warn};
use serde_json::{json, Value};
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::from_utf8;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use base64::Engine;
use opentelemetry::{global, KeyValue};
use tokio::net::TcpListener;
use uuid::Uuid;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, TracerProvider};
use tokio::signal::unix::{signal, SignalKind};
use tower_http::timeout::TimeoutLayer;

use mimalloc::MiMalloc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use crate::events::Events;
use crate::installation::Installation;
use crate::writers::parquet::parquet::process;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn shutdown_hook(sender: Sender<Events>, active: Arc<AtomicBool>, closed: Arc<AtomicBool>) {
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received SIGINT, shutting down");
            closed.store(true, Relaxed);
            drop(sender);
            loop {
                if active.load(Relaxed) {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        },
        _ = terminate.recv() => {
            warn!("Received SIGTERM, shutting down");
            closed.store(true, Relaxed);
            drop(sender);
            loop {
                if active.load(Relaxed) {
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

async fn events(State(sender): State<Sender<Events>>, extract::Json(payload): extract::Json<Events>) -> Result<(StatusCode, String), (StatusCode, String)> {
    sender.send(payload).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, "Error processing payload".to_owned()))?;
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

    let (send, mut recv) = mpsc::channel(10000);
    let active = Arc::new(AtomicBool::new(true));
    let closed = Arc::new(AtomicBool::new(false));

    let active_process = Arc::clone(&active);
    let active_closed = Arc::clone(&closed);
    tokio::spawn(async move {
        while !active_closed.load(Relaxed) {
            if let Err(err) = process(&mut recv, Arc::clone(&active_process)).await {
                error!("error processing events!: {:?}", err);
            }
        }
    });

    let app = Router::new()
        .route("/register", get(register))
        .route("/events", post(events))
        .layer(DefaultBodyLimit::disable())
        .layer(TimeoutLayer::new(Duration::from_secs(600)))
        .with_state(send.clone());

    info!(target: "bosca", "Listening on http://0.0.0.0:8009");

    axum::serve(TcpListener::bind("0.0.0.0:8009").await.unwrap(), app)
        .with_graceful_shutdown(shutdown_hook(send, active, closed))
        .await
        .unwrap();
}
