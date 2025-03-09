mod authed_subscription;
mod context;
mod datastores;
mod files;
mod graphql;
mod initialization;
mod logger;
mod models;
mod queries;
mod redis;
mod schema;
mod security;
mod shutdown_hook;
mod slugs;
mod util;
mod workflow;

use crate::files::{download, upload};
use async_graphql::extensions::apollo_persisted_queries::ApolloPersistedQueries;
use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::{
    routing::get,
    Router,
};
use log::info;
use std::env;
use std::process::exit;
use std::time::Duration;
use tokio::net::TcpListener;

use crate::context::BoscaContext;
use rustls::crypto::ring;
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;
use tower_http::timeout::TimeoutLayer;

use mimalloc::MiMalloc;
use tower_http::cors::CorsLayer;

use crate::authed_subscription::AuthGraphQLSubscription;
use crate::graphql::schema::new_schema;
use crate::initialization::content::initialize_content;
use crate::initialization::security::initialize_security;
use crate::shutdown_hook::shutdown_hook;
use crate::slugs::slug;
use crate::graphql::handlers::{graphiql_handler, graphql_handler};
use crate::initialization::telemetry::new_telemetry;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    structured_logger::Builder::with_level("info")
        .with_target_writer(
            "*",
            structured_logger::async_json::new_writer(tokio::io::stdout()),
        )
        .init();

    ring::default_provider().install_default().unwrap();

    let ctx = match BoscaContext::new().await {
        Ok(ctx) => ctx,
        Err(e) => {
            println!("{}", e.message);
            exit(1);
        }
    };

    initialize_security(&ctx).await.unwrap();
    initialize_content(&ctx).await.unwrap();

    ctx.workflow.start_monitoring_expirations();

    let telemetry = new_telemetry().unwrap();
    let persisted_queries = ApolloPersistedQueries::new(ctx.queries.cache.clone());
    let schema = new_schema(ctx.clone(), telemetry, persisted_queries);

    let upload_limit: usize = match env::var("UPLOAD_LIMIT") {
        Ok(limit) => limit.parse().unwrap(),
        _ => 2147483648,
    };

    let files = Router::new()
        .route("/upload", post(upload))
        .route("/download", get(download))
        .with_state(ctx.clone());

    let content = Router::new()
        .route("/{slug}", get(slug))
        .with_state(ctx.clone());

    let app = Router::new()
        .route("/", get(graphiql_handler))
        .nest("/files", files)
        .nest("/content", content)
        .route("/graphql", post(graphql_handler))
        .route("/graphql", get(graphql_handler))
        .route_service("/ws", AuthGraphQLSubscription::new(schema.clone(), ctx))
        .layer(DefaultBodyLimit::max(upload_limit))
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::new(Duration::from_secs(600)))
        .with_state(schema);

    info!(target: "bosca", "Listening on http://0.0.0.0:8000");

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app)
        .with_graceful_shutdown(shutdown_hook())
        .await
        .unwrap();
}
