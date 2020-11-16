//! Reimplementation of some of xsetroot(1)
use std::ffi::CString;
use std::ptr;
use x11::xlib;

/// Set the name of the root window
pub fn name(s: &str) -> Result<(), std::ffi::NulError> {
    let name = CString::new(s)?;

    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        let screen = xlib::XDefaultScreen(display);
        let window = xlib::XRootWindow(display, screen);

        xlib::XStoreName(display, window, name.as_ptr());

        // The display needs to be closed in order for it to update
        xlib::XCloseDisplay(display);
    }
}
