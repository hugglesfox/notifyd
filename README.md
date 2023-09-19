# Notifyd

Notifyd is an in memory freedesktop notification datastore.

Notifyd doesn't display any notifications but rather provides [a dbus
api](#dbus-api) to allow client applications to access and manipulate
notifications. Additonally Notifyd will remove notifications when they expire.

## DBus API

In addition to the [freedesktop notifications
api](https://developer.gnome.org/notification-spec/), notifyd also provides the
following dbus functions:

| Interface                                            | Description                                                                         | Signature       |
|------------------------------------------------------|-------------------------------------------------------------------------------------|-----------------|
| `org.freedesktop.Notifications.GetNotificationCount` | Returns the amount of notifications in the queue                                    | -> u            |
| `org.freedesktop.Notifications.GetNotificationQueue` | Returns an array of the current notifications id, app name, summary, body and hints | -> a(usssa)     |

**Note:** Types specified after the `->` denote the return type of the function.

