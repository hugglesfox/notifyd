use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use zvariant::derive::Type;

/// A DBus safe notification
#[derive(PartialEq, Debug, Type, Deserialize, Serialize)]
pub struct DbusNotification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
}

/// A notification
#[derive(PartialEq, Debug)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    expire_timeout: Option<NaiveDateTime>,
}

impl Notification {
    pub fn new(
        id: u32,
        app_name: &str,
        summary: &str,
        body: &str,
        expire_timeout: Option<i32>,
    ) -> Self {
        Self {
            id,
            app_name: app_name.into(),
            summary: summary.into(),
            body: body.into(),
            expire_timeout: expire_timeout
                .map(|v| Utc::now().naive_utc() + Duration::milliseconds(v as i64)),
        }
    }

    /// Check to see if the notification has expired.
    ///
    /// Expired notifications should be automatically removed from the
    /// notifications queue.
    pub fn expired(&self) -> bool {
        if let Some(v) = self.expire_timeout {
            return v - Utc::now().naive_utc() <= Duration::seconds(0);
        }

        false
    }

    /// Convert to a Dbus safe notificatoin
    pub fn to_dbus(&self) -> DbusNotification {
        DbusNotification {
            id: self.id,
            app_name: self.app_name.to_string(),
            summary: self.summary.to_string(),
            body: self.body.to_string(),
        }
    }
}

/// A trait which implements a few helper methods on the notification queue
pub trait Notifications {
    /// Get the index of a notification with the given id
    fn index_notification_by_id(&self, id: u32) -> Option<usize>;

    /// Get the index of a given notification
    fn index_notification(&self, notification: &Notification) -> Option<usize>;

    /// Get a notification with a certain id
    fn get_notification(&self, id: u32) -> Option<&Notification>;

    /// Replace a notification with a certain id
    ///
    /// Returns the replaced notification
    fn replace_notification(&mut self, id: u32, replacement: Notification) -> Option<Notification>;

    /// Remove a notification with a certain id
    ///
    /// Returns the removed notification
    fn remove_notification(&mut self, id: u32) -> Option<Notification>;

    /// Add a notification
    ///
    /// Errors if a notification with the same id already exists
    fn push_notification(&mut self, notification: Notification) -> Result<(), String>;
}

impl Notifications for Vec<Notification> {
    fn index_notification_by_id(&self, id: u32) -> Option<usize> {
        self.iter().rposition(|n| n.id == id)
    }

    fn index_notification(&self, notification: &Notification) -> Option<usize> {
        self.iter()
            .enumerate()
            .filter(|(_, n)| n == &notification)
            .map(|(i, _)| i)
            .next()
    }

    fn get_notification(&self, id: u32) -> Option<&Notification> {
        self.iter().filter(|n| n.id == id).next()
    }

    fn replace_notification(
        &mut self,
        id: u32,
        mut replacement: Notification,
    ) -> Option<Notification> {
        self.index_notification_by_id(id).and_then(|i| {
            let last = self.len() - 1;
            replacement.id = id;

            self.push(replacement);
            self.swap(i, last);
            self.pop()
        })
    }

    fn remove_notification(&mut self, id: u32) -> Option<Notification> {
        self.index_notification_by_id(id)
            .and_then(|i| Some(self.remove(i)))
    }

    fn push_notification(&mut self, notification: Notification) -> Result<(), String> {
        if let Some(n) = self.get_notification(notification.id) {
            return Err(format!("Notification with id {} already exists", n.id));
        }

        Ok(self.push(notification))
    }
}
