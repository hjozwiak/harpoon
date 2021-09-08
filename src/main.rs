use std::time::Duration;

use dbus::{blocking::Connection, channel::MatchingReceiver, message::MatchRule, Error, Message};
/// A monitoring program
/// For now, I have it set to write out all the messages.
/// I need to figure out how to filter out only things that are a11y specific.
/// From what I can tell, the thing is org.a11y.Bus but not sure where to go from there.

fn main() {
    let connection = Connection::new_session().expect("I failed to connect to the session. bus");
    let mut rule = MatchRule::new();
    let proxy = connection.with_proxy(
        "org.freedesktop.DBus",
        "/org/freedesktop/dbus",
        Duration::from_millis(5000),
    );
    let result: Result<(), Error> = proxy.method_call(
        "org.freedesktop.Dbus.Monitoring",
        "BecomeMonitor",
        (vec![rule.match_str()], 0u32),
    );
    if result.is_ok() {
        connection.start_receive(
            rule,
            Box::new(|msg, _| {
                handle_message(&msg);
                true
            }),
        );
    } else {
        rule.eavesdrop = true;
        connection
            .add_match(rule, |_: (), _, msg| {
                handle_message(msg);
                true
            })
            .expect("match failed.");
    }
    loop {
        connection.process(Duration::from_millis(1000)).unwrap();
    }
}

// Print out a message
fn handle_message(msg: &Message) {
    println!("Received {:?}.", msg);
}
