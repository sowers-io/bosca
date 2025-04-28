use std::sync::Arc;
use tokio::sync::broadcast;
use crate::api::service::api::Notification;

#[derive(Clone)]
pub struct NotificationService {
    tx: Arc<broadcast::Sender<Notification>>,
}

impl NotificationService {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<Notification>(500);
        Self {
            tx: Arc::new(tx),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Notification> {
        self.tx.subscribe()
    }

    pub fn notify(&self, notification: Notification) {
        let _ = self.tx.send(notification);
    }
}
