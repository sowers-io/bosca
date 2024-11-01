mod events;
mod events_sync;
mod installation;

use axum::extract::{DefaultBodyLimit, State};
use axum::routing::post;
use axum::{extract, response::{self, IntoResponse}, routing::get, Router};
use std::str::FromStr;
use http::{HeaderMap, HeaderName, HeaderValue};
use log::{info, warn};
use serde_json::{json, Value};
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::from_utf8;
use std::sync::Arc;
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
use crate::events::Events;
use crate::installation::Installation;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn shutdown_hook() {
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received SIGINT, shutting down");
            // loop {
            //     if RUNNING_BACKGROUND.load(Relaxed) > 0 {
            //         tokio::time::sleep(Duration::from_millis(100)).await;
            //     } else {
            //         break
            //     }
            // }
        },
        _ = terminate.recv() => {
            warn!("Received SIGTERM, shutting down");
            // loop {
            //     if RUNNING_BACKGROUND.load(Relaxed) > 0 {
            //         tokio::time::sleep(Duration::from_millis(100)).await;
            //     } else {
            //         break
            //     }
            // }
        }
    }
}

async fn register() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_str("Content-Type").unwrap(), HeaderValue::from_str("application/json").unwrap());
    (headers, json!(Installation::new()).to_string())
}

async fn events(extract::Json(payload): extract::Json<Events>) {

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

    let app = Router::new()
        .route("/register", get(register))
        .route("/events", post(events))
        .layer(DefaultBodyLimit::disable())
        .layer(TimeoutLayer::new(Duration::from_secs(600)));

    info!(target: "bosca", "Listening on http://0.0.0.0:8009");

    axum::serve(TcpListener::bind("0.0.0.0:8009").await.unwrap(), app)
        .with_graceful_shutdown(shutdown_hook())
        .await
        .unwrap();
}
