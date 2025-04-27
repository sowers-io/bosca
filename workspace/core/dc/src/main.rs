//! Bosca Distributed Cache (DC) Service
//! 
//! This service provides a distributed cache implementation using raft-rs for cluster
//! management and moka for in-memory caching. It exposes an API using axum and protobuf
//! for communication.

use std::net::SocketAddr;
use std::sync::Arc;
use std::env;

use axum::{
    routing::get,
    Router,
};
use tokio::sync::broadcast;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod cache;
mod cluster;
mod api;
mod notification;
mod proto;

/// Get the host address from environment variables
fn get_host() -> String {
    env::var("DC_HOST").unwrap_or_else(|_| {
        info!("No host specified in environment, using default 0.0.0.0");
        "0.0.0.0".to_string()
    })
}

/// Get the port from environment variables
fn get_port() -> u16 {
    match env::var("DC_PORT") {
        Ok(port_str) => {
            match port_str.parse::<u16>() {
                Ok(port) => {
                    info!("Using port {} from environment", port);
                    port
                },
                Err(_) => {
                    info!("Invalid port in environment variable DC_PORT: {}, using default 3000", port_str);
                    3000
                }
            }
        },
        Err(_) => {
            info!("No port specified in environment, using default 3000");
            3000
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting Bosca Distributed Cache (DC) service");

    // Create notification channel
    let (notification_tx, _) = broadcast::channel(100);
    let notification_tx = Arc::new(notification_tx);

    // Initialize the cluster with auto-discovery
    let cluster = cluster::init_cluster().await;
    info!("Cluster initialized with node ID {}", cluster.node_id());

    // Initialize the cache
    let cache_service = cache::CacheService::new(cluster.clone(), notification_tx.clone());

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api::router(cache_service));

    // Get host and port from environment variables
    let host = get_host();
    let port = get_port();

    // Parse the host string into a socket address
    let host_parts: Vec<&str> = host.split('.').collect();
    let host_bytes = if host_parts.len() == 4 {
        match (host_parts[0].parse::<u8>(), host_parts[1].parse::<u8>(), 
               host_parts[2].parse::<u8>(), host_parts[3].parse::<u8>()) {
            (Ok(a), Ok(b), Ok(c), Ok(d)) => [a, b, c, d],
            _ => [0, 0, 0, 0]
        }
    } else {
        [0, 0, 0, 0]
    };

    // Create the socket address
    let addr = SocketAddr::from((host_bytes, port));
    info!("Listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server started, listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Bosca DC service is running"
}
