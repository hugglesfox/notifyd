use crate::notification::{DbusNotification, Notification};
use log::warn;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use zbus::dbus_interface;
use zvariant::Value;

/// DBus notification interface
///
/// See https://developer.gnome.org/notification-spec/ for more information.
pub struct Interface {
    notifications: Arc<Mutex<Vec<Notification>>>,
}

impl Interface {
    pub fn new(notifications: Arc<Mutex<Vec<Notification>>>) -> Self {
        Self { notifications }
    }
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Interface {
    fn get_capabilites(&self) -> &[&str] {
        &["body", "persistence"]
    }

    fn notify(
        &mut self,
        app_name: &str,
        replaced_id: u32,
        _app_icon: &str,
        summary: &str,
        body: &str,
        _actions: Vec<&str>,
        _hints: HashMap<&str, Value>,
        expire_timeout: i32,
    ) -> u32 {
        let mut notifications = self
            .notifications
            .lock()
            .expect("Unable to get lock on notification queue");

        let id = match replaced_id {
            0 => notifications.last().map(|v| v.id + 1).unwrap_or(1),
            n => n,
        };

        // If the expiry timeout is an invalid value (e.g. -10) then it'll be
        // parsed as expiring 10 milliseconds in the past therefore will
        // instantly expire after being created
        let expire_timeout = match expire_timeout {
            // Default timeout of 5 seconds
            -1 => Some(5000),
            0 => None,
            v => Some(v),
        };

        let notification = Notification::new(id, app_name, summary, body, expire_timeout);

        use crate::notification::Notifications as _;
        match replaced_id {
            0 => {
                notifications.push_notification(notification).ok();
                id
            }
            n => {
                notifications.replace_notification(n, notification);
                replaced_id
            }
        }
    }

    fn close_notification(&mut self, id: u32) {
        let mut notifications = self
            .notifications
            .lock()
            .expect("Unable to get lock on notification queue");

        use crate::notification::Notifications as _;
        if let None = notifications.remove_notification(id) {
            warn!("Tried to close unknown notification with id {}", id)
        };

        self.notification_closed(id, 3).unwrap();
    }

    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notifyd", "", env!("CARGO_PKG_VERSION"), "1.2")
    }

    fn get_notification_count(&self) -> u32 {
        self.notifications
            .lock()
            .expect("Unable to get lock on notification queue")
            .len()
            .try_into()
            .expect("Unable to get notification count")
    }

    fn get_notification_queue(&self) -> Vec<DbusNotification> {
        self.notifications
            .lock()
            .expect("Unable to get lock on notification queue")
            .iter()
            .map(|n| n.to_dbus())
            .collect()
    }

    #[dbus_interface(signal)]
    fn notification_closed(&self, id: u32, reason: u32) -> zbus::Result<()>;
}
