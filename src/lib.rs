//! # Notifyd
//!
//! Notifyd is a lightweight notification daemon designed to provide a simple
//! notification management interface for other programs to interact with.
//!
//! This library provides a [zbus client proxy](zbus::dbus_proxy) for interacting with the daemon.
//! See the [zbus book](https://dbus2.github.io/zbus/client.html) for more information.
//!
//! ## Example
//!
//! ```no_run
//! # use zbus::Connection;
//! # use notifyd::NotifydProxy;
//! #
//! # #[async_std::main]
//! # async fn main() -> zbus::Result<()> {
//! let connection = Connection::session().await?;
//! let notifyd = NotifydProxy::new(&connection).await?;
//!
//! // Get all the notifications as a hashmap
//! let notifications = notifyd.get_notifications().await?;
//!
//! // Close the notification with an id of 1
//! notifyd.close_notification(1).await?;
//! #
//! # Ok(())
//! # }
//! ```

use zbus::{dbus_proxy, Result};
use std::collections::HashMap;

pub mod notification;
pub use crate::notification::Notification;

#[dbus_proxy(
    default_service = "org.freedesktop.Notifications",
    interface = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifyd {
    /// Close a notification with the given ID
    fn close_notification(&self, id: u32) -> Result<()>;

    /// Get all notifications as a hashmap where the key is the id of the notification
    fn get_notifications(&self) -> Result<HashMap<u32, Notification>>;

    #[dbus_proxy(signal)]
    fn new_notification(&self, id: u32) -> Result<()>;
}
