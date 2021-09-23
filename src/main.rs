mod generated;
use generated::accessible::AccessibleProxy;
use generated::bus::BusProxy;
use zbus::{Connection, Result};

fn main() {
    let accessibility_bus = get_accessibility_bus(
        Connection::new_session().expect("Problem connecting to session bus."),
    )
    .expect("Could not connect to the accessibility bus.");
    let root = AccessibleProxy::new_for(
        &accessibility_bus,
        "org.a11y.atspi.Registry",
        "/org/a11y/atspi/accessible/root",
    )
    .expect("Error creating proxy object.");
    let children = root.get_children().expect("Error getting children");
    for child in children {
        println!("{}: {:?}", child.0, child.1);
    }
}

fn get_accessibility_bus(con: Connection) -> Result<Connection> {
    let proxy = BusProxy::new(&con).expect("Problem creating the bus proxy.");
    let actual_bus = match proxy.get_address() {
        Ok(addr) => addr,
        Err(e) => return Err(e),
    };
    Connection::new_for_address(&actual_bus, true)
}
