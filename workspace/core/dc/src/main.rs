use std::env;
use std::net::SocketAddr;
use std::process::exit;
use std::sync::Arc;

use crate::cluster::node::Node;
use axum::{routing::get, Router};
use tokio::sync::broadcast;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

mod api;
mod cache;
mod cluster;
mod notification;
mod proto;

fn get_id() -> Uuid {
    let id = env::var("DC_ID").unwrap_or_else(|_| {
        info!("No DC_ID");
        exit(1);
    });
    Uuid::parse_str(&id).unwrap()
}

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
        Ok(port_str) => match port_str.parse::<u16>() {
            Ok(port) => {
                info!("Using port {} from DC_PORT", port);
                port
            }
            Err(_) => {
                info!(
                    "Invalid port in environment variable DC_PORT: {}, using default 5000",
                    port_str
                );
                5000
            }
        },
        Err(_) => {
            info!("No port specified in environment, using default 5000");
            5000
        }
    }
}

fn get_broadcast_port() -> u16 {
    match env::var("DC_BROADCAST_PORT") {
        Ok(port_str) => match port_str.parse::<u16>() {
            Ok(port) => {
                info!("Using port {} from DC_BROADCAST_PORT", port);
                port
            }
            Err(_) => {
                info!("Invalid port in environment variable DC_BROADCAST_PORT: {}, using default 5001", port_str);
                5001
            }
        },
        Err(_) => {
            info!("No port specified in environment, using default 5001");
            5001
        }
    }
}

fn get_receive_port() -> u16 {
    match env::var("DC_RECEIVE_PORT") {
        Ok(port_str) => {
            match port_str.parse::<u16>() {
                Ok(port) => {
                    info!("Using port {} from DC_RECEIVE_PORT", port);
                    port
                }
                Err(_) => {
                    info!("Invalid port in environment variable DC_RECEIVE_PORT: {}, using default 5002", port_str);
                    5002
                }
            }
        }
        Err(_) => {
            info!("No port specified in environment, using default 5002");
            5002
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    info!("Starting Bosca Distributed Cache (DC) service");

    // Create notification channel
    let (notification_tx, _) = broadcast::channel(100);
    let notification_tx = Arc::new(notification_tx);

    let host = get_host();
    let port = get_port();

    let id = get_id();
    // Initialize the cluster with auto-discovery
    let node = Node::new(id, host.clone(), port);
    let cluster =
        cluster::cluster::Cluster::new(node, get_receive_port(), get_broadcast_port()).await;

    cluster.broadcast().await;

    // Initialize the cache
    let cache_service = cache::CacheService::new(cluster, notification_tx.clone());

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", api::router(cache_service));

    // Parse the host string into a socket address
    let host_parts: Vec<&str> = host.split('.').collect();
    let host_bytes = if host_parts.len() == 4 {
        match (
            host_parts[0].parse::<u8>(),
            host_parts[1].parse::<u8>(),
            host_parts[2].parse::<u8>(),
            host_parts[3].parse::<u8>(),
        ) {
            (Ok(a), Ok(b), Ok(c), Ok(d)) => [a, b, c, d],
            _ => [0, 0, 0, 0],
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
