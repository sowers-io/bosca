//! API layer for the distributed cache service
//! 
//! This module provides the HTTP API for interacting with the cache service.
//! It uses axum for routing and handling requests.

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{sse::{Event, Sse}, IntoResponse, Response},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;

use crate::cache::CacheService;
use crate::notification::NotificationService;

/// Request to create a new cache
#[derive(Debug, Deserialize)]
pub struct CreateCacheRequest {
    /// Name of the cache
    pub name: String,
    /// Maximum capacity of the cache
    pub max_capacity: u64,
}

/// Response for a successful cache creation
#[derive(Debug, Serialize)]
pub struct CreateCacheResponse {
    /// ID of the created cache
    pub cache_id: String,
}

/// Request to put a value in the cache
#[derive(Debug, Deserialize)]
pub struct PutRequest {
    /// Value to store
    pub value: Vec<u8>,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

/// Creates the API router
pub fn router(cache_service: Arc<CacheService>) -> Router {
    // Create notification service
    let notification_service = NotificationService::new(cache_service.notification_tx.clone());

    // Create router with routes
    Router::new()
        .route("/caches", post(create_cache))
        .route("/caches/{cache_id}/values/{key}", get(get_value))
        .route("/caches/{cache_id}/values/{key}", post(put_value))
        .route("/caches/{cache_id}/values/{key}", delete(delete_value))
        .route("/caches/{cache_id}", delete(clear_cache))
        .route("/caches/{cache_id}/notifications", get(subscribe_notifications))
        .with_state((cache_service, notification_service))
}

/// Handler for creating a new cache
async fn create_cache(
    State((cache_service, _)): State<(Arc<CacheService>, Arc<NotificationService>)>,
    Json(request): Json<CreateCacheRequest>,
) -> Response {
    match cache_service.create_cache(request.name, request.max_capacity).await {
        Ok(cache_id) => {
            let response = CreateCacheResponse { cache_id };
            (StatusCode::CREATED, Json(response)).into_response()
        },
        Err(error) => {
            let response = ErrorResponse { error };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// Handler for getting a value from the cache
async fn get_value(
    State((cache_service, _)): State<(Arc<CacheService>, Arc<NotificationService>)>,
    Path((cache_id, key)): Path<(String, String)>,
) -> Response {
    match cache_service.get(&cache_id, &key).await {
        Some(value) => (StatusCode::OK, value).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Handler for putting a value in the cache
async fn put_value(
    State((cache_service, _)): State<(Arc<CacheService>, Arc<NotificationService>)>,
    Path((cache_id, key)): Path<(String, String)>,
    Json(request): Json<PutRequest>,
) -> Response {
    match cache_service.put(&cache_id, key, request.value).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(error) => {
            let response = ErrorResponse { error };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// Handler for deleting a value from the cache
async fn delete_value(
    State((cache_service, _)): State<(Arc<CacheService>, Arc<NotificationService>)>,
    Path((cache_id, key)): Path<(String, String)>,
) -> Response {
    match cache_service.delete(&cache_id, &key).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => {
            let response = ErrorResponse { error };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// Handler for clearing a cache
async fn clear_cache(
    State((cache_service, _)): State<(Arc<CacheService>, Arc<NotificationService>)>,
    Path(cache_id): Path<String>,
) -> Response {
    match cache_service.clear(&cache_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => {
            let response = ErrorResponse { error };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// Handler for subscribing to notifications
async fn subscribe_notifications(
    State((_, notification_service)): State<(Arc<CacheService>, Arc<NotificationService>)>,
    Path(cache_id): Path<String>,
) -> Response {
    // Create a new subscription
    let mut rx = notification_service.subscribe();

    // Create a stream that filters notifications for the specified cache
    let stream = async_stream::stream! {
        while let Ok(notification) = rx.recv().await {
            if notification.cache_id == cache_id {
                let json = serde_json::to_string(&notification).unwrap_or_default();
                let event = Event::default().data(json);
                yield Ok::<_, Infallible>(event);
            }
        }
    };

    // Return the stream as a server-sent events response
    Sse::new(stream).into_response()
}
