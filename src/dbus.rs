use crate::xsetroot;
use std::collections::HashMap;
use zbus::dbus_interface;
use zvariant::Value;

/// The dumbest DBus notification server
///
/// See https://developer.gnome.org/notification-spec/ for more information.
pub struct Interface {
    current_id: u64,
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
        _replaced_id: u64,
        _app_icon: &str,
        summary: &str,
        _body: &str,
        _actions: Vec<&str>,
        _hints: HashMap<&str, Value>,
        _expire_timeout: i32,
    ) -> u64 {
        xsetroot::name(format!("{}: {}", app_name, summary).as_str());

        // Just cos it needs an incrementing number
        self.current_id.wrapping_add(1)
    }

    fn close_notification(&self, id: u64) {}

    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notifyd", "", env!("CARGO_PKG_VERSION"), "1.2")
    }
}
