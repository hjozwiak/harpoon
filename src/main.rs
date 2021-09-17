mod generated;
use generated::bus::BusProxy;
use log::{debug, error, info};
use simple_log::quick;
use zbus::Connection;
fn main() {
    quick().expect("Error getting the log server setup.");
    debug!("Starting program.");
    let connection = Connection::new_session().expect("Could not connect to bus.");
    let proxy = generated::bus::BusProxy::new(&connection).expect("Error obtaining proxy object.");
    let actual_bus = proxy.get_address();
    match actual_bus {
        Ok(address) => debug!("Found the atspi bus at {}", address),
        Err(e) => error!("Problem getting the bus. {}", e),
    }
}
