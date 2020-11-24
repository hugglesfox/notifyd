use chrono::prelude::*;
use chrono::Duration;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zvariant::derive::Type;
use zvariant::Value;

/// A DBus safe notification
#[derive(PartialEq, Debug, Type, Deserialize, Serialize)]
pub struct DbusNotification<'a> {
    pub id: u32,
    pub app_name: &'a str,
    pub summary: &'a str,
    pub body: &'a str,
    pub hints: HashMap<&'a str, Value<'a>>,
}

/// A notification
#[derive(PartialEq, Debug)]
pub struct Notification<'a> {
    pub id: u32,
    pub app_name: &'a str,
    pub summary: &'a str,
    pub body: &'a str,
    pub hints: HashMap<&'a str, Value<'a>>,
    expire_timeout: Option<NaiveDateTime>,
}

impl<'a> Notification<'a> {
    pub fn new(
        id: u32,
        app_name: &'a str,
        summary: &'a str,
        body: &'a str,
        hints: HashMap<&'a str, Value<'a>>,
        expire_timeout: i32,
    ) -> Self {
        // If the expiry timeout is an invalid value (e.g. -10) then it'll be
        // parsed as expiring 10 milliseconds in the past therefore will
        // instantly expire after being created
        let expire_timeout = match expire_timeout {
            // Default timeout is 60 seconds
            -1 => {
                match hints.get("urgency") {
                    // Critical urgency
                    Some(Value::U32(2)) => None,
                    // Normal urgency
                    Some(Value::U32(1)) => Some(120000),
                    // Low urgency
                    Some(Value::U32(0)) => Some(60000),
                    // Err
                    _ => {
                        debug!("Unknown urgency. defaulting to timeout of 60000 milliseconds");
                        Some(60000)
                    }
                }
            }
            0 => None,
            v => Some(v),
        };

        Self {
            id,
            app_name,
            summary,
            body,
            hints,
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
            app_name: self.app_name,
            summary: self.summary,
            body: self.body,
            hints: self.hints,
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

impl Notifications for Vec<Notification<'_>> {
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
