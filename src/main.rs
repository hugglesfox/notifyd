//! # Notifyd
//!
//! Notifyd is a lightweight notification daemon designed to provide a simple
//! notification management interface for other programs to interact with.
//!
//! *Note:* Notification expires are checked when a new dbus messages are received.

extern crate pretty_env_logger;

use log::info;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use zbus::fdo;

mod dbus;
mod notification;

use notification::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let notification_queue: Arc<Mutex<Vec<Notification>>> = Arc::default();

    let connection = zbus::Connection::new_session()?;
    fdo::DBusProxy::new(&connection)?.request_name(
        "org.freedesktop.Notifications",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;

    let mut server = zbus::ObjectServer::new(&connection);

    let iface = dbus::Interface::new(Arc::clone(&notification_queue));
    server.at(&"/org/freedesktop/Notifications".try_into()?, iface)?;

    // Close expired notifications
    thread::spawn(move || {
        let connection = zbus::Connection::new_session().expect("Failed to connect to dbus");
        loop {
            {
                let mut notifications = notification_queue
                    .lock()
                    .expect("Unable to lock notification queue");

                let expired: Vec<u32> = notifications
                    .iter()
                    .filter(|n| n.expired())
                    .map(|n| n.id)
                    .collect();

                for id in expired {
                    use notification::Notifications as _;
                    info!("Removing expired notification {}", id);
                    notifications.remove_notification(id);

                    connection
                        .emit_signal(
                            None,
                            "/org/freedesktop/Notifications",
                            "org.freedesktop.Notifications",
                            "NotificationClosed",
                            &(id, 1),
                        )
                        .expect("Unable to send signal");
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    });

    loop {
        if let Err(err) = server.try_handle_next() {
            eprintln!("{}", err);
        }
    }
}
