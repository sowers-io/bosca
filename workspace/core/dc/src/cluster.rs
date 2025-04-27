//! Simplified cluster management module
//! 
//! This module provides a simplified implementation of cluster management
//! for the distributed cache service.

use std::sync::Arc;
use std::env;
use std::process::Command;
use std::net::{ToSocketAddrs, SocketAddr};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::time::{timeout, Duration};
use tracing::{info, warn, error};
use uuid::Uuid;

/// Represents the cluster state and operations
pub struct Cluster {
    /// Node ID
    node_id: u64,
    /// Peer node IDs
    peers: RwLock<Vec<u64>>,
    /// Pending operations
    pending_ops: RwLock<Vec<Vec<u8>>>,
}

/// Result type for cluster operations
type ClusterResult<T> = Result<T, String>;

impl Cluster {
    /// Creates a new cluster instance
    pub async fn new(node_id: u64, peers: Vec<u64>) -> Self {
        info!("Creating new cluster instance with node ID {} and peers {:?}", node_id, peers);

        Self {
            node_id,
            peers: RwLock::new(peers),
            pending_ops: RwLock::new(Vec::new()),
        }
    }

    /// Proposes a change to the cluster state
    pub async fn propose(&self, data: Vec<u8>) -> ClusterResult<()> {
        info!("Proposing change to cluster state: {} bytes", data.len());

        // In a simplified implementation, we just store the operation
        let mut pending_ops = self.pending_ops.write().await;
        pending_ops.push(data);

        Ok(())
    }

    /// Processes cluster events
    pub async fn tick(&self) {
        // In a simplified implementation, this is a no-op
    }

    /// Returns the node ID
    pub fn node_id(&self) -> u64 {
        self.node_id
    }

    /// Returns the peer node IDs
    pub async fn peers(&self) -> Vec<u64> {
        self.peers.read().await.clone()
    }

    /// Adds a peer to the cluster
    pub async fn add_peer(&self, peer_id: u64) -> ClusterResult<()> {
        let mut peers = self.peers.write().await;
        if !peers.contains(&peer_id) {
            peers.push(peer_id);
            info!("Added peer {} to cluster", peer_id);
        }
        Ok(())
    }

    /// Removes a peer from the cluster
    pub async fn remove_peer(&self, peer_id: u64) -> ClusterResult<()> {
        let mut peers = self.peers.write().await;
        if let Some(index) = peers.iter().position(|&id| id == peer_id) {
            peers.remove(index);
            info!("Removed peer {} from cluster", peer_id);
        }
        Ok(())
    }
}

/// Generates or reads the node ID
fn get_node_id() -> u64 {
    // First check if node ID is specified in environment variables
    match env::var("DC_NODE_ID") {
        Ok(id) => {
            match id.parse::<u64>() {
                Ok(node_id) => {
                    info!("Using node ID {} from environment", node_id);
                    return node_id;
                },
                Err(_) => {
                    warn!("Invalid node ID in environment variable DC_NODE_ID: {}, will auto-generate", id);
                }
            }
        },
        Err(_) => {
            info!("No node ID specified in environment, will auto-generate");
        }
    }

    // Try to get hostname
    let hostname = match Command::new("hostname").output() {
        Ok(output) => {
            if output.status.success() {
                match String::from_utf8(output.stdout) {
                    Ok(hostname) => {
                        let hostname = hostname.trim();
                        info!("Using hostname {} for node ID generation", hostname);
                        hostname.to_string()
                    },
                    Err(_) => {
                        warn!("Failed to parse hostname output, will use UUID");
                        "".to_string()
                    }
                }
            } else {
                warn!("Failed to get hostname, will use UUID");
                "".to_string()
            }
        },
        Err(_) => {
            warn!("Failed to execute hostname command, will use UUID");
            "".to_string()
        }
    };

    // Generate node ID from hostname or UUID
    let node_id = if !hostname.is_empty() {
        // Simple hash of hostname to generate a u64
        let mut hash: u64 = 5381;
        for c in hostname.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u64);
        }
        // Ensure it's not 0 (reserved) and fits in u64
        let node_id = hash.max(1);
        info!("Generated node ID {} from hostname", node_id);
        node_id
    } else {
        // Use UUID v4 as fallback
        let uuid = Uuid::new_v4();
        let node_id = (uuid.as_u128() % (u64::MAX as u128)) as u64;
        // Ensure it's not 0 (reserved)
        let node_id = node_id.max(1);
        info!("Generated node ID {} from UUID", node_id);
        node_id
    };

    node_id
}

/// Performs DNS lookup to discover peers
async fn discover_peers_via_dns() -> Vec<u64> {
    // Get the service name from environment or use default
    let service_name = env::var("DC_SERVICE_NAME").unwrap_or_else(|_| {
        info!("No service name specified in environment, using default 'dc'");
        "dc".to_string()
    });

    // Try to resolve the service name using DNS
    let lookup_addr = format!("{}:{}", service_name, 3000);
    info!("Attempting to discover peers via DNS lookup of {}", lookup_addr);

    // Clone the lookup_addr to ensure it lives long enough
    let lookup_addr_clone = lookup_addr.clone();
    match timeout(Duration::from_secs(5), tokio::net::lookup_host(lookup_addr_clone)).await {
        Ok(Ok(addrs)) => {
            // Convert addresses to node IDs
            let peers: Vec<u64> = addrs
                .map(|addr| {
                    // Extract IP and convert to a node ID
                    let ip = addr.ip();
                    let ip_bytes = match ip {
                        std::net::IpAddr::V4(ipv4) => ipv4.octets().to_vec(),
                        std::net::IpAddr::V6(ipv6) => ipv6.octets().to_vec(),
                    };

                    // Simple hash of IP to generate a u64
                    let mut hash: u64 = 5381;
                    for b in ip_bytes {
                        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u64);
                    }
                    // Ensure it's not 0 (reserved)
                    hash.max(1)
                })
                .collect();

            if peers.is_empty() {
                warn!("DNS lookup succeeded but no peers found, will use default peers");
                vec![1, 2, 3]
            } else {
                info!("Discovered peers via DNS: {:?}", peers);
                peers
            }
        },
        _ => {
            warn!("Failed to discover peers via DNS, will use default peers");
            vec![1, 2, 3]
        }
    }
}

/// Gets peer IDs from environment variables or discovers them automatically
async fn get_peers() -> Vec<u64> {
    // First check if peers are specified in environment variables
    match env::var("DC_PEERS") {
        Ok(peers_str) => {
            let peers: Vec<u64> = peers_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();

            if peers.is_empty() {
                warn!("No valid peer IDs found in environment variable DC_PEERS: {}, will try auto-discovery", peers_str);
            } else {
                info!("Using peer IDs {:?} from environment", peers);
                return peers;
            }
        },
        Err(_) => {
            info!("No peer IDs specified in environment, will try auto-discovery");
        }
    }

    // Try to discover peers via DNS
    discover_peers_via_dns().await
}

/// Initializes the cluster
pub async fn init_cluster() -> Arc<Cluster> {
    // Read configuration from environment variables or auto-generate
    let node_id = get_node_id();
    let peers = get_peers().await;

    info!("Initializing cluster with node ID {} and peers {:?}", node_id, peers);

    let cluster = Cluster::new(node_id, peers).await;
    Arc::new(cluster)
}
