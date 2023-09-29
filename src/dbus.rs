use crate::store::NotificationStore;
use crate::notification::Hints;
use crate::notification::Notification;
use async_std::sync::Arc;
use std::collections::HashMap;
use log::{debug, warn};
use zbus::SignalContext;
use zbus::dbus_interface;

pub const BUS_NAME: &str = "org.freedesktop.Notifications";
pub const OBJ_PATH: &str = "/org/freedesktop/Notifications";


/// DBus notification interface
///
/// See https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html for more information.
pub struct Interface {
    id: u32,
    notifications: Arc<NotificationStore>, 
}

impl Interface {
    pub fn new(notifications: Arc<NotificationStore>) -> Self {
        Self {
            id: 0,
            notifications,
        }
    }

}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Interface {
    /// Get the capabilities of the notification daemon
    ///
    /// TODO: Get clients to specify their capabilites with a function. Use a signal to notify
    /// clients that they should send their capabilites
    fn get_capabilities(&self) -> &[&str] {
        &["body", "persistence"]
    }

    /// Create a new notification
    async fn notify(
        &mut self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: Hints,
        expire_timeout: i32,
    ) -> u32 {
        self.id = self.id.wrapping_add(1);
        self.notifications.insert(
            self.id,
            Notification::new(
                app_name,
                replaces_id,
                app_icon,
                summary,
                body,
                actions,
                hints,
                expire_timeout,
            ),
        ).await;

        Self::new_notification(&ctxt).await.unwrap();

        debug!("Created notification {:?}", self.id);

        self.id
    }

    /// Delete a notification
    async fn close_notification(
        &mut self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        id: u32,
    ) {
        if let None = self.notifications.remove(id).await {
            warn!("Tried to close non-existant notification with id {}", id)
        };

        Self::notification_closed(&ctxt, id).await.unwrap();
    }

    /// Get information about the notification server
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notifyd", "", env!("CARGO_PKG_VERSION"), "1.2")
    }

    /// Get notification with given id
    async fn get_notification(&self, id: u32) -> Result<Notification, zbus::fdo::Error> {
        self.notifications.get(id).await.ok_or(zbus::fdo::Error::InvalidArgs("Notification with given ID does not exist".to_string()))
    }

    /// Get notifications
    async fn get_notifications(&self) -> HashMap<u32, Notification> {
        self.notifications.to_map().await
    }

    #[dbus_interface(signal)]
    async fn new_notification(ctx: &SignalContext<'_>) -> zbus::Result<()> {}

    #[dbus_interface(signal)]
    async fn notification_closed(ctx: &SignalContext<'_>, id: u32) -> zbus::Result<()> {}
}
