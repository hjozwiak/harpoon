use std::time::Duration;
mod generated;
use dbus::blocking::Connection;
fn main() {
    // register on the user bus.
    let connection = Connection::new_session().expect("Failed to connect to the session bus.");
    let proxy = connection.with_proxy(
        "org.a11y.atspi.Bus",
        "/org/a11y/atspi/bus",
        Duration::new(5, 0),
    );
    use generated::OrgA11yBus;
    let new_address = proxy.get_address().expect("Failed to get the address.");
    println!(new_address);
}
