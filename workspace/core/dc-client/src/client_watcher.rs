use futures_util::TryStreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::api::WatchParams;
use kube::{
    api::{Api, WatchEvent},
    Client as KubeClient, Error,
};
use log::{error, info};
use std::pin::pin;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tonic::transport::channel::Change;
use tonic::transport::Endpoint;

pub async fn watch(namespace: String, port: u16, sender: &Sender<Change<String, Endpoint>>) -> Result<(), Error> {
    let kube_client = KubeClient::try_default().await?;
    let service_name =
        std::env::var("DISTRIBUTED_CACHE_SERVICE").unwrap_or_else(|_| "cache".to_string());
    let field_selector = format!("metadata.name={}", service_name);
    let wp = WatchParams::default().fields(&field_selector);
    let pods: Api<Pod> = Api::namespaced(kube_client, &namespace);
    let mut pods_stream = pin!(pods.watch(&wp, "0").await?);
    while let Some(event) = pods_stream.try_next().await.unwrap_or(None) {
        info!("pod event: {:?}", event);
        match event {
            WatchEvent::Added(pod) | WatchEvent::Modified(pod) => {
                if let Some(status) = &pod.status {
                    if let Some(pod_ip) = &status.pod_ip {
                        let pod_url = format!("http://{}:{}", pod_ip, port);
                        info!("discovered cache server: {}", pod_url);
                        let new_endpoint = match Endpoint::new(pod_url) {
                            Ok(ep) => ep
                                .connect_timeout(Duration::from_secs(3))
                                .timeout(Duration::from_secs(3))
                                .keep_alive_timeout(Duration::from_secs(3)),
                            Err(e) => {
                                error!("Failed to create endpoint: {}", e);
                                continue;
                            }
                        };
                        if let Err(e) = sender
                            .send(Change::Insert(
                                pod.metadata.name.clone().unwrap_or_default(),
                                new_endpoint,
                            ))
                            .await
                        {
                            error!("Failed to update channel endpoint: {}", e);
                        }
                    }
                }
            }
            WatchEvent::Deleted(pod) => {
                if let Err(e) = sender
                    .send(Change::Remove(pod.metadata.name.unwrap_or_default()))
                    .await
                {
                    error!("Failed to remove channel endpoint: {}", e);
                }
            }
            _ => {}
        }
    }
    Ok(())
}
