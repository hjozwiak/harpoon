// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 4b6cabc)
// DO NOT EDIT

use crate::Accessible;
use crate::ComponentLayer;
use crate::CoordType;
use crate::Point;
use crate::Rect;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "AtspiComponent")]
    pub struct Component(Interface<ffi::AtspiComponent>);

    match fn {
        type_ => || ffi::atspi_component_get_type(),
    }
}

pub const NONE_COMPONENT: Option<&Component> = None;

pub trait ComponentExt: 'static {
    #[doc(alias = "atspi_component_contains")]
    fn contains(&self, x: i32, y: i32, ctype: CoordType) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_component_get_accessible_at_point")]
    #[doc(alias = "get_accessible_at_point")]
    fn accessible_at_point(&self, x: i32, y: i32, ctype: CoordType) -> Result<Option<Accessible>, glib::Error>;

    #[doc(alias = "atspi_component_get_alpha")]
    #[doc(alias = "get_alpha")]
    fn alpha(&self) -> Result<f64, glib::Error>;

    #[doc(alias = "atspi_component_get_extents")]
    #[doc(alias = "get_extents")]
    fn extents(&self, ctype: CoordType) -> Result<Rect, glib::Error>;

    #[doc(alias = "atspi_component_get_layer")]
    #[doc(alias = "get_layer")]
    fn layer(&self) -> Result<ComponentLayer, glib::Error>;

    #[doc(alias = "atspi_component_get_mdi_z_order")]
    #[doc(alias = "get_mdi_z_order")]
    fn mdi_z_order(&self) -> Result<libc::c_short, glib::Error>;

    #[doc(alias = "atspi_component_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self, ctype: CoordType) -> Result<Point, glib::Error>;

    #[doc(alias = "atspi_component_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> Result<Point, glib::Error>;

    #[doc(alias = "atspi_component_grab_focus")]
    fn grab_focus(&self) -> Result<(), glib::Error>;

    //#[doc(alias = "atspi_component_scroll_to")]
    //fn scroll_to(&self, type_: /*Ignored*/ScrollType) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_component_scroll_to_point")]
    fn scroll_to_point(&self, coords: CoordType, x: i32, y: i32) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_component_set_extents")]
    fn set_extents(&self, x: i32, y: i32, width: i32, height: i32, ctype: CoordType) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_component_set_position")]
    fn set_position(&self, x: i32, y: i32, ctype: CoordType) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_component_set_size")]
    fn set_size(&self, width: i32, height: i32) -> Result<(), glib::Error>;
}

impl<O: IsA<Component>> ComponentExt for O {
    fn contains(&self, x: i32, y: i32, ctype: CoordType) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_component_contains(self.as_ref().to_glib_none().0, x, y, ctype.into_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn accessible_at_point(&self, x: i32, y: i32, ctype: CoordType) -> Result<Option<Accessible>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_component_get_accessible_at_point(self.as_ref().to_glib_none().0, x, y, ctype.into_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn alpha(&self) -> Result<f64, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_component_get_alpha(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn extents(&self, ctype: CoordType) -> Result<Rect, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_component_get_extents(self.as_ref().to_glib_none().0, ctype.into_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn layer(&self) -> Result<ComponentLayer, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_component_get_layer(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn mdi_z_order(&self) -> Result<libc::c_short, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_component_get_mdi_z_order(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn position(&self, ctype: CoordType) -> Result<Point, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_component_get_position(self.as_ref().to_glib_none().0, ctype.into_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn size(&self) -> Result<Point, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_component_get_size(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn grab_focus(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_component_grab_focus(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn scroll_to(&self, type_: /*Ignored*/ScrollType) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:atspi_component_scroll_to() }
    //}

    fn scroll_to_point(&self, coords: CoordType, x: i32, y: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_component_scroll_to_point(self.as_ref().to_glib_none().0, coords.into_glib(), x, y, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_extents(&self, x: i32, y: i32, width: i32, height: i32, ctype: CoordType) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_component_set_extents(self.as_ref().to_glib_none().0, x, y, width, height, ctype.into_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_position(&self, x: i32, y: i32, ctype: CoordType) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_component_set_position(self.as_ref().to_glib_none().0, x, y, ctype.into_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_size(&self, width: i32, height: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_component_set_size(self.as_ref().to_glib_none().0, width, height, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Component")
    }
}