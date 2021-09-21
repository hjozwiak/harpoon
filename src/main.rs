mod generated;
use generated::bus::BusProxy;
use zbus::{Connection, Result};

fn main() {
    let accessibility_bus = get_accessibility_bus(
        Connection::new_session().expect("Problem connecting to session bus."),
    )
    .expect("Could not connect to the accessibility bus.");
}

fn get_accessibility_bus(con: Connection) -> Result<Connection> {
    let proxy = BusProxy::new(&con).expect("Problem creating the bus proxy.");
    let actual_bus = match proxy.get_address() {
        Ok(addr) => addr,
        Err(e) => return Err(e),
    };
    Connection::new_for_address(&actual_bus, true)
}
