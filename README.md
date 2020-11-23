# Notifyd

Notifyd is a lightweight notification daemon.

Notifyd doesn't display any notifications but rather provides [additional dbus
functions](#DBus API) to allow the creation of clients.

## DBus API

Along with the usual [freedesktop notifications
api](https://developer.gnome.org/notification-spec/), notifyd also implements
some custom functions to for use by notification clients.

| Interface                                            | Description                                                                  | Signature  |
|------------------------------------------------------|------------------------------------------------------------------------------|------------|
| `org.freedesktop.Notifications.GetNotificationCount` | Returns the amount of notifications in the queue                             | -> u       |
| `org.freedesktop.Notifications.GetNotificationQueue` | Returns an array of the current notifications id, app name, summary and body | -> a(usss) |

**Note:** Types specified after the `->` denote the return type of the function.

