use crate::api::service::api::distributed_cache_server::DistributedCache;
use crate::api::service::api::{
    ClearCacheRequest, ClearCacheResponse, CreateCacheResponse, DeleteValueRequest,
    DeleteValueResponse, Empty, GetNodesResponse, GetValueRequest, GetValueResponse, Node,
    Notification, NotificationType, PutValueRequest, PutValueResponse,
    SubscribeNotificationsRequest,
};
use crate::cache::cache_service::CacheService;
use crate::cluster::Cluster;
use crate::notification::NotificationService;
use std::pin::Pin;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use tracing::{debug, error, info};

pub mod api {
    tonic::include_proto!("bosca.dc");
}

pub struct DistributedCacheImpl {
    cluster: Cluster,
    cache: CacheService,
    notifications: NotificationService,
}

impl DistributedCacheImpl {
    pub fn new(cluster: Cluster, cache: CacheService, notifications: NotificationService) -> Self {
        Self {
            cluster,
            cache,
            notifications,
        }
    }
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Notification, Status>> + Send>>;

#[async_trait::async_trait]
impl DistributedCache for DistributedCacheImpl {
    async fn create_cache(
        &self,
        request: Request<api::CreateCacheRequest>,
    ) -> Result<Response<CreateCacheResponse>, Status> {
        let req = request.get_ref();
        if let Err(e) = self
            .cache
            .create_cache(&req.name, req.max_capacity, req.ttl, req.tti, true)
            .await
        {
            return Err(Status::internal(e.to_string()));
        }
        Ok(Response::new(CreateCacheResponse {
            cache: req.name.clone(),
        }))
    }

    async fn get_nodes(&self, _: Request<Empty>) -> Result<Response<GetNodesResponse>, Status> {
        Ok(Response::new(GetNodesResponse {
            nodes: self.cluster.get_nodes().await,
        }))
    }

    async fn get_value(
        &self,
        request: Request<GetValueRequest>,
    ) -> Result<Response<GetValueResponse>, Status> {
        let req = request.get_ref();
        let value = self
            .cache
            .get(&req.cache, &req.key)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetValueResponse { value }))
    }

    async fn put_value(
        &self,
        request: Request<PutValueRequest>,
    ) -> Result<Response<PutValueResponse>, Status> {
        let req = request.get_ref();
        self.cache
            .put(&req.cache, req.key.clone(), req.value.clone(), true)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(PutValueResponse { success: true }))
    }

    async fn delete_value(
        &self,
        request: Request<DeleteValueRequest>,
    ) -> Result<Response<DeleteValueResponse>, Status> {
        let req = request.get_ref();
        self.cache
            .delete(&req.cache, &req.key, true)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(DeleteValueResponse { success: true }))
    }

    async fn clear_cache(
        &self,
        request: Request<ClearCacheRequest>,
    ) -> Result<Response<ClearCacheResponse>, Status> {
        let req = request.get_ref();
        self.cache
            .clear(&req.cache, true)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(ClearCacheResponse { success: true }))
    }

    async fn replicate(&self, request: Request<Notification>) -> Result<Response<Empty>, Status> {
        let req = request.get_ref();
        debug!("replicating: {:?}", req);
        let notification_type: NotificationType =
            NotificationType::try_from(req.notification_type).unwrap();
        match notification_type {
            NotificationType::CacheCreated => {
                if let Some(create) = &req.create {
                    self.cache
                        .create_cache(
                            &req.cache.clone(),
                            create.max_capacity,
                            create.ttl,
                            create.tti,
                            false,
                        )
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                }
            }
            NotificationType::ValueUpdated => {
                if let Some(k) = &req.key {
                    if let Some(v) = &req.value {
                        let node = self.cluster.get_node(k).await;
                        if node.is_none() || node.unwrap().id == self.cluster.node.id {
                            self.cache
                                .put(&req.cache, k.clone(), v.clone(), false)
                                .await
                                .map_err(|e| Status::internal(e.to_string()))?;
                        }
                    }
                }
            }
            NotificationType::ValueDeleted => {
                if let Some(k) = &req.key {
                    self.cache
                        .delete(&req.cache, k, false)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;
                }
            }
            NotificationType::CacheCleared => {
                self.cache
                    .clear(&req.cache, false)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }
            NotificationType::NodeFound => {
                if let Some(n) = &req.node {
                    self.cluster.register(n.clone(), false).await;
                }
            }
            NotificationType::NodeLost => {
                if let Some(n) = &req.node {
                    self.cluster.deregister(n.clone(), false).await;
                }
            }
        }
        Ok(Response::new(Empty {}))
    }

    async fn join(&self, request: Request<Node>) -> Result<Response<Empty>, Status> {
        let req = request.get_ref();
        info!("joining: {:?}", req);
        self.cluster.register(req.clone(), true).await;
        Ok(Response::new(Empty {}))
    }

    async fn ping(&self, _: Request<Node>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }

    type SubscribeNotificationsStream = ResponseStream;

    async fn subscribe_notifications(
        &self,
        _: Request<SubscribeNotificationsRequest>,
    ) -> Result<Response<Self::SubscribeNotificationsStream>, Status> {
        let (tx, rx) = channel(500);
        let mut subscribe = self.notifications.subscribe();
        tokio::spawn(async move {
            while let Ok(item) = subscribe.recv().await {
                match tx.send(Ok(item)).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("error sending notification: {}", e);
                        break;
                    }
                }
            }
        });
        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::SubscribeNotificationsStream
        ))
    }
}
