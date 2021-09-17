//! # DBus interface proxy for: `org.a11y.atspi.Action`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Action.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://zeenix.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Action")]
trait Action {
    /// DoAction method
    fn do_action(&self, index: i32) -> zbus::Result<bool>;

    /// GetActions method
    fn get_actions(&self) -> zbus::Result<Vec<(String, String, String)>>;

    /// GetDescription method
    fn get_description(&self, index: i32) -> zbus::Result<String>;

    /// GetKeyBinding method
    fn get_key_binding(&self, index: i32) -> zbus::Result<String>;

    /// GetLocalizedName method
    fn get_localized_name(&self, index: i32) -> zbus::Result<String>;

    /// GetName method
    fn get_name(&self, index: i32) -> zbus::Result<String>;

    /// NActions property
    #[dbus_proxy(property)]
    fn nactions(&self) -> zbus::Result<i32>;
}
