// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 4b6cabc)
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Range(Boxed<ffi::AtspiRange>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::atspi_range_get_type(), ptr as *mut _) as *mut ffi::AtspiRange,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::atspi_range_get_type(), ptr as *mut _),
        type_ => || ffi::atspi_range_get_type(),
    }
}