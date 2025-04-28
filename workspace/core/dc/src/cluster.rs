use crate::api::service::api::distributed_cache_client::DistributedCacheClient;
use crate::api::service::api::{Node, Notification, NotificationType};
use crate::notification::NotificationService;
use hashring::HashRing;
use prost::Message;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::info;
use tracing::warn;
use tracing::{debug, error};

#[derive(Debug)]
struct Peer {
    node: Node,
    failures: AtomicI32,
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.node.id == other.node.id
    }
}

impl Eq for Peer {}

impl Eq for Node {}

#[derive(Clone)]
pub struct Cluster {
    pub node: Node,
    peers: Arc<RwLock<Vec<Peer>>>,
    hash: Arc<RwLock<HashRing<Peer>>>,
    notifications: NotificationService,
    receive_port: u16,
    broadcast_port: u16,
}

impl Cluster {
    pub async fn new(
        node: Node,
        receive_port: u16,
        broadcast_port: u16,
        notifications: NotificationService,
    ) -> Self {
        let cluster = Self {
            node: node.clone(),
            peers: Arc::new(RwLock::new(vec![Peer {
                node,
                failures: AtomicI32::new(0),
            }])),
            hash: Arc::new(RwLock::new(HashRing::new())),
            notifications,
            receive_port,
            broadcast_port,
        };
        let notify_cluster = cluster.clone();
        tokio::spawn(async move {
            let mut subscribe = notify_cluster.notifications.subscribe();
            while let Ok(notification) = subscribe.recv().await {
                if let Some(node) = &notification.node {
                    if node.id != notify_cluster.node.id {
                        continue;
                    }
                }
                debug!("received notification: {:?}", notification);
                notify_cluster.notify(notification).await;
            }
        });
        let health_check = cluster.clone();
        tokio::spawn(async move {
            loop {
                health_check.healthcheck().await;
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        });
        cluster
    }

    async fn healthcheck(&self) {
        let mut failures = Vec::new();
        {
            let peers = self.peers.read().await;
            for peer in peers.iter() {
                if peer.node.id == self.node.id {
                    continue;
                }
                let mut failed = false;
                if let Ok(mut client) = DistributedCacheClient::connect(format!(
                    "http://{}:{}",
                    peer.node.ip, peer.node.port
                ))
                .await
                {
                    if let Err(_) = client.ping(self.node.clone()).await {
                        failed = true;
                    }
                } else {
                    failed = true;
                }
                if failed {
                    warn!("peer failed: {:?}", peer.node);
                    if peer.failures.fetch_add(1, Relaxed) > 3 {
                        warn!("peer being marked as failure: {:?}", peer.node);
                        failures.push(peer.node.clone());
                    }
                }
            }
        }
        if failures.len() > 0 {
            for failure in failures {
                self.deregister(failure, true).await;
            }
            let peers = self.peers.read().await;
            info!("peers after failure: {:?}", peers);
        }
    }

    pub async fn deregister(&self, node: Node, notify: bool) {
        let mut peers = self.peers.write().await;
        peers.retain(|p| p.node.id != node.id);
        if notify {
            self.notifications.notify(Notification {
                notification_type: NotificationType::NodeLost.into(),
                node: Some(node.clone()),
                ..Default::default()
            });
        }
    }

    pub async fn register(&self, node: Node, notify: bool) {
        let peer = Peer {
            failures: AtomicI32::new(0),
            node: node.clone(),
        };
        let found = {
            let peers = self.peers.read().await;
            peers.contains(&peer)
        };
        if !found {
            debug!("discovered: {:?}", node);
            {
                let mut peers = self.peers.write().await;
                peers.push(peer);
                debug!("peers: {:?}", peers);
            }
            if notify {
                self.notifications.notify(Notification {
                    notification_type: NotificationType::NodeFound.into(),
                    node: Some(node.clone()),
                    ..Default::default()
                });
            }
            if let Ok(mut client) =
                DistributedCacheClient::connect(format!("http://{}:{}", node.ip, node.port)).await
            {
                if let Err(e) = client.join(self.node.clone()).await {
                    error!("failed to join: {}", e);
                }
            }
        }
    }

    async fn notify(&self, message: Notification) {
        for peer in self.peers.read().await.iter() {
            if self.node.id == peer.node.id {
                continue;
            }
            if let Ok(mut client) = DistributedCacheClient::connect(format!(
                "http://{}:{}",
                peer.node.ip, peer.node.port
            ))
            .await
            {
                let request = tonic::Request::new(message.clone());
                if let Err(e) = client.replicate(request).await {
                    error!("failed to replicate notification: {}", e);
                }
            }
        }
    }

    pub async fn broadcast(&self) {
        let node_id_bytes = self.node.encode_length_delimited_to_vec();
        let port = self.broadcast_port;
        tokio::spawn(async move {
            let sock = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap())
                .await
                .unwrap();
            sock.set_broadcast(true).unwrap();
            sock.join_multicast_v4(
                "224.0.0.0".parse::<Ipv4Addr>().unwrap(),
                "0.0.0.0".parse::<Ipv4Addr>().unwrap(),
            )
            .unwrap();
            loop {
                if let Err(e) = sock
                    .send_to(&node_id_bytes, format!("224.0.0.0:{port}"))
                    .await
                {
                    error!("Failed to send broadcast: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
        let cluster = self.clone();
        let port = self.receive_port;
        tokio::spawn(async move {
            let sock = UdpSocket::bind(format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap())
                .await
                .unwrap();
            sock.set_broadcast(true).unwrap();
            sock.join_multicast_v4(
                "224.0.0.0".parse::<Ipv4Addr>().unwrap(),
                "0.0.0.0".parse::<Ipv4Addr>().unwrap(),
            )
            .unwrap();
            let mut buf = [0; 512];
            loop {
                let _ = sock.recv_from(&mut buf).await.unwrap();
                let b = &buf[..];
                match Node::decode_length_delimited(b) {
                    Ok(node) => {
                        cluster.register(node, true).await;
                    }
                    Err(e) => {
                        error!("Failed to decode node: {}", e);
                    }
                }
            }
        });
    }
}
