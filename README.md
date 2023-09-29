# Notifyd

Notifyd is an in memory freedesktop notification datastore.

Notifyd doesn't display any notifications but rather provides a dbus api to
allow client applications to access and manipulate notifications. Additonally
Notifyd will remove notifications when they expire.

## Client Usage

In addition to implementing the freedesktop dbus methods and signals, notifyd
provides two extra methods `GetNotification` and `GetNotifications` as well as
the signal `NewNotification` for use in creating notifyd clients. See the
notifyd rust library documentation for further usage details (TODO, link to
docsrs).

```rust
use zbus::Connection;
use notifyd::NotifydProxy;

#[async_std::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let notifyd = NotifydProxy::new(&connection).await?;

    // Get all the notifications as a hashmap
    let notifications = notifyd.get_notifications().await?;

    // Close the notification with an id of 1
    notifyd.close_notification(1).await?;

    Ok(())
}
```
