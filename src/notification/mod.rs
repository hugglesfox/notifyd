use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use zbus::zvariant::Type;

pub mod hints;

pub use hints::Hints;

fn epoch_now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH")
}

/// A notification
#[derive(Clone, Serialize, Deserialize, Type)]
pub struct Notification {
    timestamp: u64,
    app_name: String,
    replaces_id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: Hints,
    expire_timeout: i32,
}

impl Notification {
    /// Create a new notification
    ///
    /// The types taken by this function are designed to somewhat match the
    /// types used by the notify dbus function.
    pub fn new(
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: Hints,
        expire_timeout: i32,
    ) -> Self {
        // TODO: configurable default timeout
        let expiry = match expire_timeout {
            -1 => 2000,
            _ => expire_timeout,
        };

        Self {
            timestamp: epoch_now().as_secs(),
            app_name,
            replaces_id,
            app_icon,
            summary,
            body,
            actions,
            hints,
            expire_timeout: expiry,
        }
    }

    /// Check to see if the notification has expired.
    ///
    /// Expired notifications should be automatically closed by notifyd.
    pub fn is_expired(&self) -> bool {
        if self.expire_timeout == 0 {
            return false;
        }

        Duration::from_secs(self.timestamp) + Duration::from_millis(self.expire_timeout as u64)
            <= epoch_now()
    }
}
