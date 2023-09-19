//! # Notifyd
//!
//! Notifyd is a lightweight notification daemon designed to provide a simple
//! notification management interface for other programs to interact with.

extern crate pretty_env_logger;

use log::{debug, error, info};
use std::sync::{Arc, Mutex};
use zbus::{ConnectionBuilder, Result};

mod dbus;
mod notification;

use dbus::{BUS_NAME, OBJ_PATH};
use notification::Notification;

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_custom_env("NOTIFYD_LOG");
    info!("Starting notifyd...");

    let notification_queue: Arc<Mutex<Vec<Notification>>> = Arc::default();

    let connection = ConnectionBuilder::session()?
        .name(BUS_NAME)?
        .serve_at(
            OBJ_PATH,
            dbus::Interface::new(notification_queue),
        )?
        .build()
        .await?;

    info!("Listening on {} {}", BUS_NAME, OBJ_PATH);

    loop {
        std::future::pending::<()>().await;
    }
}
