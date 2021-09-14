// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 520ff74)
// DO NOT EDIT

use crate::Device;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtspiDeviceLegacy")]
    pub struct DeviceLegacy(Object<ffi::AtspiDeviceLegacy, ffi::AtspiDeviceLegacyClass>) @extends Device;

    match fn {
        type_ => || ffi::atspi_device_legacy_get_type(),
    }
}

impl DeviceLegacy {
    #[doc(alias = "atspi_device_legacy_new")]
    pub fn new() -> DeviceLegacy {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::atspi_device_legacy_new())
        }
    }
}

impl Default for DeviceLegacy {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DEVICE_LEGACY: Option<&DeviceLegacy> = None;

impl fmt::Display for DeviceLegacy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceLegacy")
    }
}
