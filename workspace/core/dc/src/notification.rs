//! Notification system for cache updates
//! 
//! This module provides the notification system for cache updates.
//! It allows clients to subscribe to notifications for specific caches.

use serde::{Serialize, Deserialize};
use tokio::sync::broadcast;
use std::sync::Arc;

/// Types of notifications that can be sent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    /// A new cache was created
    CacheCreated,
    /// A value was updated in the cache
    ValueUpdated,
    /// A value was deleted from the cache
    ValueDeleted,
    /// The cache was cleared
    CacheCleared,
}

/// Notification message sent when a cache is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// ID of the cache that was updated
    pub cache_id: String,
    /// Type of notification
    pub notification_type: NotificationType,
    /// Key that was updated (if applicable)
    pub key: Option<String>,
}

/// Notification service for subscribing to cache updates
pub struct NotificationService {
    notification_tx: Arc<broadcast::Sender<Notification>>,
}

impl NotificationService {
    /// Creates a new notification service
    pub fn new(notification_tx: Arc<broadcast::Sender<Notification>>) -> Arc<Self> {
        Arc::new(Self {
            notification_tx,
        })
    }
    
    /// Subscribes to notifications for a specific cache
    pub fn subscribe(&self) -> broadcast::Receiver<Notification> {
        self.notification_tx.subscribe()
    }
    
    /// Sends a notification
    pub fn notify(&self, notification: Notification) {
        let _ = self.notification_tx.send(notification);
    }
}