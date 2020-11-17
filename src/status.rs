use crate::xsetroot;
use chrono::prelude::*;
use std::thread;
use std::time::Duration;
use zbus::{dbus_proxy, fdo};

#[dbus_proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/devices/DisplayDevice"
)]
trait Upowerd {
    #[dbus_proxy(property)]
    fn time_to_empty(&self) -> fdo::Result<i64>;

    #[dbus_proxy(property)]
    fn time_to_full(&self) -> fdo::Result<i64>;
}

/// Get the current time to fully charged/discharged
fn battery(upower: &UpowerdProxy) -> Result<NaiveTime, Box<dyn std::error::Error + Sync + Send>> {
    // Time to empty is 0 whilst charging and vice versa so we can get either the
    // time to full or time to empty, in seconds, by adding them together
    let time = upower.time_to_empty()? + upower.time_to_full()?;
    Ok(NaiveTime::from_hms(0, 0, time as u32))
}

/// Set the status bar to something useful every 5 minutes
///
/// This will clear the current notification
pub fn display() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let connection = zbus::Connection::new_system()?;
    let upower = UpowerdProxy::new(&connection)?;

    loop {
        let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let battery = battery(&upower)?.format("%H:%M:%S").to_string();

        xsetroot::name(format!("{} | {}", battery, time).as_str());
        thread::sleep(Duration::from_secs(300));
    }
}
