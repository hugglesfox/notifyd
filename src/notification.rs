use chrono::prelude::*;
use chrono::Duration;
use log::debug;
use serde::{Deserialize, Serialize};
use zvariant::derive::Type;

#[derive(PartialEq, Debug, Type, Deserialize, Serialize, Clone)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

/// A DBus safe notification
#[derive(PartialEq, Debug, Type, Deserialize, Serialize)]
pub struct DbusNotification {
    id: u32,
    app_name: String,
    summary: String,
    body: String,
    urgency: Urgency,
}

/// A notification
#[derive(PartialEq, Debug)]
pub struct Notification {
    pub id: u32,
    app_name: String,
    summary: String,
    body: String,
    urgency: Urgency,
    expire_timeout: Option<NaiveDateTime>,
}

impl Notification {
    pub fn new(
        id: u32,
        app_name: &str,
        summary: &str,
        body: &str,
        urgency: Urgency,
        expire_timeout: i32,
    ) -> Self {
        // If the expiry timeout is an invalid value (e.g. -10) then it'll be
        // parsed as expiring 10 milliseconds in the past therefore will
        // instantly expire after being created
        let expire_timeout = match expire_timeout {
            // Default timeout is 60 seconds
            -1 => match urgency {
                Urgency::Critical => None,
                Urgency::Normal => Some(120000),
                Urgency::Low => Some(60000),
            },
            0 => None,
            v => Some(v),
        };

        Self {
            id,
            app_name: app_name.into(),
            summary: summary.into(),
            body: body.into(),
            urgency,
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
            app_name: self.app_name.to_owned(),
            summary: self.summary.to_owned(),
            body: self.body.to_owned(),
            urgency: self.urgency.clone(),
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

    /// Remove a notification with a certain id
    ///
    /// If the notification being replaced doesn't exist, the new notification
    /// will just be added with the given id.
    fn remove_notification(&mut self, id: u32) -> Option<Notification>;

    /// Add a notification
    ///
    /// If a notification with the given id already exists, the new notification
    /// will replace it.
    fn push_notification(&mut self, notification: Notification);
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

    fn remove_notification(&mut self, id: u32) -> Option<Notification> {
        self.index_notification_by_id(id)
            .and_then(|i| Some(self.remove(i)))
    }

    fn push_notification(&mut self, notification: Notification) {
        if let Some(n) = self.remove_notification(notification.id) {
            debug!("Notification with id {} already exists, replacing", n.id);
        }

        self.push(notification);
    }
}
