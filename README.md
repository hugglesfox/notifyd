# Notifyd

Notifyd is a lightweight notification daemon designed for window managers
which use xsetroot(1) in order to customise the status bar (such as dwm).

Notifyd only implements just enough of the freedesktop notifications
protocol to just barely work so therefore has no support for things such as
expiry timeouts, icons, queues, etc. All it does is display the latest
notification and every 5 minutes show a clock and battery time to
empty/full.
