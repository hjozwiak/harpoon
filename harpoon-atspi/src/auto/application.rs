// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 4b6cabc)
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtspiApplication")]
    pub struct Application(Object<ffi::AtspiApplication, ffi::AtspiApplicationClass>);

    match fn {
        type_ => || ffi::atspi_application_get_type(),
    }
}

impl Application {}

pub const NONE_APPLICATION: Option<&Application> = None;

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Application")
    }
}
