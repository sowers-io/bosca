use futures_util::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::Api,
    Client as KubeClient, Error,
};
use kube_runtime::watcher::Event;
use kube_runtime::watcher;
use log::{error, info};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tonic::transport::channel::Change;
use tonic::transport::Endpoint;

async fn add_endpoint(pod: &Pod, port: u16, sender: &Sender<Change<String, Endpoint>>) -> Result<(), Error> {
    info!("pod applied: {:?}", pod.metadata.name);
    if let Some(status) = &pod.status {
        if let Some(pod_ip) = &status.pod_ip {
            let pod_url = format!("http://{pod_ip}:{port}");
            info!("discovered cache server: {}", pod_url);
            let new_endpoint = match Endpoint::new(pod_url) {
                Ok(ep) => ep
                    .connect_timeout(Duration::from_secs(3))
                    .timeout(Duration::from_secs(3))
                    .keep_alive_timeout(Duration::from_secs(3)),
                Err(e) => {
                    error!("Failed to create endpoint: {}", e);
                    return Ok(());
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
    Ok(())
}

pub async fn watch(
    namespace: String,
    port: u16,
    sender: &Sender<Change<String, Endpoint>>,
) -> Result<(), Error> {
    let kube_client = KubeClient::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(kube_client, &namespace);
    let mut stream = watcher(pods, watcher::Config::default().labels("app=cache"))
        .into_stream()
        .boxed();
    let mut pending = Vec::new();
    while let Some(event) = stream.try_next().await.unwrap_or(None) {
        match event {
            Event::Apply(pod) => {
                add_endpoint(&pod, port, sender).await?;
            }
            Event::Delete(pod) => {
                info!("pod deleted: {:?}", pod.metadata.name);
                if let Err(e) = sender
                    .send(Change::Remove(pod.metadata.name.unwrap_or_default()))
                    .await
                {
                    error!("Failed to remove channel endpoint: {}", e);
                }
            }
            Event::Init => {
                pending.clear();
            }
            Event::InitApply(pod) => {
                pending.push(pod);
            }
            Event::InitDone => {
                for pod in pending.iter() {
                    add_endpoint(pod, port, sender).await?;
                }
                pending.clear();
            }
        }
    }
    Ok(())
}
