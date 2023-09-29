extern crate pretty_env_logger;

use log::{debug, info};
use zbus::{ConnectionBuilder, Result};
use notifyd::NotifydProxy;

mod dbus;
mod notification;
mod store;

use store::NotificationStore;
use dbus::{BUS_NAME, OBJ_PATH};

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_custom_env("NOTIFYD_LOG");
    info!("Starting notifyd...");

    let notifications = NotificationStore::new();

    let interface = dbus::Interface::new(notifications.clone());

    let connection = ConnectionBuilder::session()?
        .name(BUS_NAME)?
        .serve_at(OBJ_PATH, interface)?
        .build()
        .await?;

    info!("Listening on {} {}", BUS_NAME, OBJ_PATH);

    let client = NotifydProxy::new(&connection).await.unwrap();

    loop {
        for id in notifications.expired_ids().await {
            debug!("Closing expired notification {}", id);
            client.close_notification(id).await.ok();
        }
    }
}
