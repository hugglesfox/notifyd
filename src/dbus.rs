use crate::notification::DbusNotification;
use crate::notification::Notification;
use crate::notification::Urgency;
use log::{debug, warn};
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use zbus::dbus_interface;
use zbus::zvariant::Value;
use zbus::Result;
use zbus::SignalContext;

pub const BUS_NAME: &str = "org.freedesktop.Notifications";
pub const OBJ_PATH: &str = "/org/freedesktop/Notifications";

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
    /// Get the capabilities of the notification daemon
    async fn get_capabilities(&self) -> &[&str] {
        &["body", "persistence"]
    }

    /// Create a new notification
    async fn notify(
        &mut self,
        app_name: &str,
        replaced_id: u32,
        _app_icon: &str,
        summary: &str,
        body: &str,
        _actions: Vec<&str>,
        hints: HashMap<&str, Value<'_>>,
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

        let urgency = match hints.get("urgency") {
            Some(Value::I32(2)) => Urgency::Critical,
            Some(Value::I32(1)) => Urgency::Normal,
            Some(Value::I32(0)) => Urgency::Low,
            // Err
            _ => {
                debug!("Unknown urgency. Defaulting to low");
                Urgency::Low
            }
        };

        let notification = Notification::new(id, app_name, summary, body, urgency, expire_timeout);

        use crate::notification::Notifications as _;
        notifications.push_notification(notification);

        debug!("{:?}", notifications);

        id
    }

    /// Delete a notification
    async fn close_notification(
        &mut self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        id: u32,
    ) {
        let mut notifications = self
            .notifications
            .lock()
            .expect("Unable to get lock on notification queue");

        use crate::notification::Notifications as _;
        if let None = notifications.remove_notification(id) {
            warn!("Tried to close unknown notification with id {}", id)
        };

        Self::notification_closed(&ctxt, id);
    }

    /// Get information about the notification server
    async fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notifyd", "", env!("CARGO_PKG_VERSION"), "1.2")
    }

    /// Get the amount of notifications
    async fn get_notification_count(&self) -> u32 {
        self.notifications
            .lock()
            .expect("Unable to get lock on notification queue")
            .len()
            .try_into()
            .expect("Unable to get notification count")
    }

    /// Get all the notifications
    async fn get_notification_queue(&self) -> Vec<DbusNotification> {
        self.notifications
            .lock()
            .expect("Unable to get lock on notification queue")
            .iter()
            .map(|n| n.to_dbus())
            .collect()
    }

    #[dbus_interface(signal)]
    async fn notification_closed(ctx: &SignalContext<'_>, id: u32) -> Result<()> {}
}
