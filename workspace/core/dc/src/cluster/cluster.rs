use crate::cluster::node::Node;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::error;
use uuid::Uuid;

#[derive(Clone)]
pub struct Cluster {
    node: Node,
    peers: Arc<RwLock<Vec<Node>>>,
    receive_port: u16,
    broadcast_port: u16
}

impl Cluster {
    pub async fn new(node: Node, receive_port: u16, broadcast_port: u16) -> Self {
        Self {
            node,
            peers: Arc::new(RwLock::new(Vec::new())),
            receive_port,
            broadcast_port
        }
    }

    pub async fn broadcast(&self) {
        let node_id_bytes = self.node.id.to_bytes_le();
        let port = self.broadcast_port;
        tokio::spawn(async move {
            let sock = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())
                .await
                .unwrap();
            sock.set_broadcast(true).unwrap();
            sock.join_multicast_v4("224.0.0.0".parse::<Ipv4Addr>().unwrap(), "0.0.0.0".parse::<Ipv4Addr>().unwrap()).unwrap();
            loop {
                if let Err(e) = sock.send_to(&node_id_bytes[0..16], format!("255.255.255.255:{}", port)).await {
                    error!("Failed to send broadcast: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        let peers = Arc::clone(&self.peers);
        let port = self.receive_port;
        tokio::spawn(async move {
            let sock = UdpSocket::bind(format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap())
                .await
                .unwrap();
            sock.set_broadcast(true).unwrap();
            sock.join_multicast_v4("224.0.0.0".parse::<Ipv4Addr>().unwrap(), "0.0.0.0".parse::<Ipv4Addr>().unwrap()).unwrap();
            let mut buf = [0; 16];
            loop {
                let (_, addr) = sock.recv_from(&mut buf).await.unwrap();
                let id = Uuid::from_slice_le(&buf[..16]).unwrap();
                let node = Node::new(id, addr.ip().to_string(), addr.port());
                let mut peers = peers.write().await;
                if !peers.contains(&node) {
                    println!("discovered: {:?} {:?}", id, node);
                    peers.push(node);
                }
            }
        });
    }
}
