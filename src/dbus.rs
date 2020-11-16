use std::collections::HashMap;
use zbus::dbus_interface;
use zvariant::Value;

/// DBus notification interface
///
/// See https://developer.gnome.org/notification-spec/ for more information
pub struct Interface {
    current_id: u32,
}

impl Default for Interface {
    fn default() -> Self {
        Self { current_id: 0 }
    }
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Interface {
    fn get_capabilites(&self) -> &[&str] {
        &[]
    }

    fn notify(
        &mut self,
        app_name: &str,
        replaced_id: u32,
        _app_icon: &str,
        summary: &str,
        _body: &str,
        _actions: Vec<&str>,
        _hints: HashMap<&str, Value>,
        expire_timeout: i32,
    ) -> u32 {
    }

    fn close_notification(&self, id: u32) {}
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notifyd", "", env!("CARGO_PKG_VERSION"), "1.2")
    }
}
