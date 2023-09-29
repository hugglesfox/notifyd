use async_std::sync::{Arc, RwLock};
use std::collections::HashMap;
use crate::notification::Notification;

/// A thread safe key value store for notifications
pub struct NotificationStore(RwLock<HashMap<u32, Notification>>);

impl NotificationStore {
    pub fn new() -> Arc<Self> {
        Arc::new(Self(RwLock::new(HashMap::new())))
    }
    pub async fn insert(&self, id: u32, notification: Notification) -> Option<Notification> {
        self.0.write().await.insert(id, notification)
    }

    pub async fn remove(&self, id: u32) -> Option<Notification> {
        self.0.write().await.remove(&id)
    }

    pub async fn get(&self, id: u32) -> Option<Notification> {
        self.0.read().await.get(&id).cloned()
    }

    pub async fn to_map(&self) -> HashMap<u32, Notification> {
        self.0.read().await.clone()
    }

    pub async fn expired_ids(&self) -> Vec<u32> {
        self.0.write().await
            .iter()
            .filter(|(_, n)| n.is_expired())
            .map(|(id, _)| *id)
            .collect()
    }
}

