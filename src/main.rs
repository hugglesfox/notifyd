//! # Notifyd
//!
//! Notifyd is a lightweight notification daemon designed for window managers
//! which use xsetroot(1) in order to customise the status bar (such as dwm).
//!
//! Notifyd only implements just enough of the freedesktop notifications
//! protocol to just barely work so therefore has no support for things such as
//! expiry timeouts, icons, queues, etc. All it does is display the latest
//! notification and every 10 seconds show a clock and battery time to
//! empty/full.

use std::convert::TryInto;
use std::thread;
use zbus::fdo;

mod dbus;
mod status;
mod xsetroot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    thread::spawn(|| status::display());

    let connection = zbus::Connection::new_session()?;
    fdo::DBusProxy::new(&connection)?.request_name(
        "org.freedesktop.Notifications",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;

    let mut server = zbus::ObjectServer::new(&connection);

    let iface = dbus::Interface::default();
    server.at(&"/org/freedesktop/Notifications".try_into()?, iface)?;

    loop {
        if let Err(err) = server.try_handle_next() {
            eprintln!("{}", err);
        }
    }
}
