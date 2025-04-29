use std::env;
use std::net::ToSocketAddrs;
use crate::api::service::api::distributed_cache_server::DistributedCacheServer;
use crate::api::service::api::Node;
use crate::api::service::DistributedCacheImpl;
use crate::cache::cache_service::CacheService;
use crate::cluster::Cluster;
use crate::notification::NotificationService;
use tonic::transport::Server;
use tonic_reflection::pb::v1::FILE_DESCRIPTOR_SET;
use tonic_reflection::server::Builder;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

mod api;
mod cache;
mod cluster;
mod notification;

fn get_id() -> String {
    let id = env::var("DC_ID").unwrap_or_else(|_| Uuid::new_v4().to_string());
    Uuid::parse_str(&id).unwrap().to_string()
}

fn get_host() -> String {
    env::var("DC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn get_port() -> u16 {
    match env::var("DC_PORT") {
        Ok(port_str) => match port_str.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                info!(
                    "Invalid port in environment variable DC_PORT: {}, using default 5000",
                    port_str
                );
                5000
            }
        },
        Err(_) => 5000,
    }
}

fn get_broadcast_port() -> u16 {
    match env::var("DC_BROADCAST_PORT") {
        Ok(port_str) => match port_str.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                info!("Invalid port in environment variable DC_BROADCAST_PORT: {}, using default 6001", port_str);
                6001
            }
        },
        Err(_) => 6001,
    }
}

fn get_receive_port() -> u16 {
    match env::var("DC_RECEIVE_PORT") {
        Ok(port_str) => {
            match port_str.parse::<u16>() {
                Ok(port) => port,
                Err(_) => {
                    info!("Invalid port in environment variable DC_RECEIVE_PORT: {}, using default 6002", port_str);
                    6002
                }
            }
        }
        Err(_) => 6002,
    }
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
    info!("Starting Bosca Distributed Cache (DC) Service");
    let host = get_host();
    let port = get_port();
    let id = get_id();
    let node = Node {
        id,
        ip: host.clone(),
        port: port as u32,
    };
    let notifications = NotificationService::new();
    let cluster = Cluster::new(
        node,
        get_receive_port(),
        get_broadcast_port(),
        notifications.clone(),
    )
    .await;
    cluster.broadcast().await;
    let cache_service = CacheService::new(cluster.clone(), notifications.clone());
    let api = DistributedCacheImpl::new(cluster, cache_service, notifications);
    let addr = format!("{host}:{port}").to_socket_addrs().unwrap().next().unwrap();
    info!("Listening on {}", addr);
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DistributedCacheServer<DistributedCacheImpl>>()
        .await;
    if let Err(e) = Server::builder()
        .add_service(reflection_service)
        .add_service(health_service)
        .add_service(DistributedCacheServer::new(api))
        .serve(addr)
        .await
    {
        error!("error running server: {}", e);
    }
}
