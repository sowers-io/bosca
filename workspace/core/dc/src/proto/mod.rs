//! Protocol buffer definitions for the distributed cache service
//! 
//! This module contains simplified types for the protobuf definitions.
//! The actual generated code is temporarily disabled to fix build issues.

// Simplified types for protobuf messages
#[derive(Debug, Clone)]
pub struct CreateCacheRequest {
    pub name: String,
    pub max_capacity: u64,
}

#[derive(Debug, Clone)]
pub struct CreateCacheResponse {
    pub cache_id: String,
}

#[derive(Debug, Clone)]
pub struct GetValueRequest {
    pub cache_id: String,
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct GetValueResponse {
    pub value: Vec<u8>,
    pub found: bool,
}

#[derive(Debug, Clone)]
pub struct PutValueRequest {
    pub cache_id: String,
    pub key: String,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PutValueResponse {
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct DeleteValueRequest {
    pub cache_id: String,
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct DeleteValueResponse {
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct ClearCacheRequest {
    pub cache_id: String,
}

#[derive(Debug, Clone)]
pub struct ClearCacheResponse {
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct SubscribeNotificationsRequest {
    pub cache_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    CacheCreated,
    ValueUpdated,
    ValueDeleted,
    CacheCleared,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub cache_id: String,
    pub notification_type: NotificationType,
    pub key: String,
}

// Convert between our internal notification types and protobuf types
impl From<crate::notification::NotificationType> for NotificationType {
    fn from(notification_type: crate::notification::NotificationType) -> Self {
        match notification_type {
            crate::notification::NotificationType::CacheCreated => NotificationType::CacheCreated,
            crate::notification::NotificationType::ValueUpdated => NotificationType::ValueUpdated,
            crate::notification::NotificationType::ValueDeleted => NotificationType::ValueDeleted,
            crate::notification::NotificationType::CacheCleared => NotificationType::CacheCleared,
        }
    }
}

impl From<NotificationType> for crate::notification::NotificationType {
    fn from(notification_type: NotificationType) -> Self {
        match notification_type {
            NotificationType::CacheCreated => crate::notification::NotificationType::CacheCreated,
            NotificationType::ValueUpdated => crate::notification::NotificationType::ValueUpdated,
            NotificationType::ValueDeleted => crate::notification::NotificationType::ValueDeleted,
            NotificationType::CacheCleared => crate::notification::NotificationType::CacheCleared,
        }
    }
}

impl From<crate::notification::Notification> for Notification {
    fn from(notification: crate::notification::Notification) -> Self {
        Self {
            cache_id: notification.cache_id,
            notification_type: notification.notification_type.into(),
            key: notification.key.unwrap_or_default(),
        }
    }
}

impl From<Notification> for crate::notification::Notification {
    fn from(notification: Notification) -> Self {
        Self {
            cache_id: notification.cache_id,
            notification_type: notification.notification_type.into(),
            key: if notification.key.is_empty() { None } else { Some(notification.key) },
        }
    }
}
