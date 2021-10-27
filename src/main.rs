use dbus::blocking::Connection;
mod generated;
fn main() {
    if let Ok(conn) = Connection::new_session() {
        println!("Got a DBus connection.");
    } else {
        println!("Failed to get a connection to DBus.");
    }
}
