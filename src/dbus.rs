use async_std::sync::Arc;
use crate::notification::{Hints, Notification};
use crate::store::NotificationStore;
use log::{debug, warn};
use serde_repr::{Serialize_repr, Deserialize_repr};
use std::collections::HashMap;
use zbus::fdo::Error;
use zbus::zvariant::Type;
use zbus::{SignalContext, dbus_interface};

pub const BUS_NAME: &str = "org.freedesktop.Notifications";
pub const OBJ_PATH: &str = "/org/freedesktop/Notifications";

/// Notification closure reason
#[derive(Serialize_repr, Deserialize_repr, Type)]
#[repr(u32)]
pub enum Reason {
    Expired = 1,
    Dismissed = 2,
    Closed = 3,
    Undefined = 4,
}

impl Reason {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Expired => "expired",
            Self::Dismissed => "dismissed by user",
            Self::Closed => "closed by CloseNotification",
            Self::Undefined => "undefined reason",
        }
    }
}

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

    async fn delete_notification(&mut self, ctxt: SignalContext<'_>, id: u32, reason: Reason) {
        if let None = self.notifications.remove(id).await {
            warn!("Tried to close non-existant notification with id {}", id)
        };

        debug!("Notification {} {}", id, reason.to_str());

        Self::notification_closed(&ctxt, id, reason).await.unwrap();
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

    /// Close a notification
    async fn close_notification(
        &mut self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        id: u32,
    ) {
        self.delete_notification(ctxt, id, Reason::Closed).await;
    }

    /// Dismiss a notification
    async fn dismiss_notification(
        &mut self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        id: u32,
    ) {
        self.delete_notification(ctxt, id, Reason::Dismissed).await;
    }

    /// Get information about the notification server
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notifyd", "freedesktop.org", env!("CARGO_PKG_VERSION"), "1.2")
    }

    /// Get notification with given id
    async fn get_notification(&self, id: u32) -> Result<Notification, Error> {
        self.notifications.get(id).await.ok_or(Error::InvalidArgs("Notification with given ID does not exist".to_string()))
    }

    /// Get notifications
    async fn get_notifications(&self) -> HashMap<u32, Notification> {
        self.notifications.to_map().await
    }

    #[dbus_interface(signal)]
    async fn new_notification(ctx: &SignalContext<'_>) -> zbus::Result<()> {}

    #[dbus_interface(signal)]
    async fn notification_closed(ctx: &SignalContext<'_>, id: u32, reason: Reason) -> zbus::Result<()> {}
}
