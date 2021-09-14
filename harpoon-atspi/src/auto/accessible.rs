// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 4b6cabc)
// DO NOT EDIT

use crate::Action;
use crate::Cache;
use crate::Collection;
use crate::Component;
use crate::Document;
use crate::EditableText;
use crate::Hyperlink;
use crate::Hypertext;
use crate::Image;
use crate::Object;
use crate::Role;
use crate::Selection;
use crate::StateSet;
use crate::Table;
use crate::TableCell;
use crate::Text;
use crate::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "AtspiAccessible")]
    pub struct Accessible(Object<ffi::AtspiAccessible, ffi::AtspiAccessibleClass>) @extends Object, @implements Action, Collection, Component, Document, EditableText, Hypertext, Image, Selection, Table, TableCell, Text, Value;

    match fn {
        type_ => || ffi::atspi_accessible_get_type(),
    }
}

pub const NONE_ACCESSIBLE: Option<&Accessible> = None;

pub trait AccessibleExt: 'static {
    #[doc(alias = "atspi_accessible_clear_cache")]
    fn clear_cache(&self);

    #[doc(alias = "atspi_accessible_get_accessible_id")]
    #[doc(alias = "get_accessible_id")]
    fn accessible_id(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_action_iface")]
    #[doc(alias = "get_action_iface")]
    fn action_iface(&self) -> Option<Action>;

    #[doc(alias = "atspi_accessible_get_application")]
    #[doc(alias = "get_application")]
    fn application(&self) -> Result<Accessible, glib::Error>;

    #[doc(alias = "atspi_accessible_get_atspi_version")]
    #[doc(alias = "get_atspi_version")]
    fn atspi_version(&self) -> Result<glib::GString, glib::Error>;

    //#[doc(alias = "atspi_accessible_get_attributes")]
    //#[doc(alias = "get_attributes")]
    //fn attributes(&self) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, glib::Error>;

    //#[doc(alias = "atspi_accessible_get_attributes_as_array")]
    //#[doc(alias = "get_attributes_as_array")]
    //fn attributes_as_array(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }, glib::Error>;

    #[doc(alias = "atspi_accessible_get_child_at_index")]
    #[doc(alias = "get_child_at_index")]
    fn child_at_index(&self, child_index: i32) -> Result<Accessible, glib::Error>;

    #[doc(alias = "atspi_accessible_get_child_count")]
    #[doc(alias = "get_child_count")]
    fn child_count(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_accessible_get_collection_iface")]
    #[doc(alias = "get_collection_iface")]
    fn collection_iface(&self) -> Option<Collection>;

    #[doc(alias = "atspi_accessible_get_component_iface")]
    #[doc(alias = "get_component_iface")]
    fn component_iface(&self) -> Option<Component>;

    #[doc(alias = "atspi_accessible_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_document_iface")]
    #[doc(alias = "get_document_iface")]
    fn document_iface(&self) -> Option<Document>;

    #[doc(alias = "atspi_accessible_get_editable_text_iface")]
    #[doc(alias = "get_editable_text_iface")]
    fn editable_text_iface(&self) -> Option<EditableText>;

    #[doc(alias = "atspi_accessible_get_hyperlink")]
    #[doc(alias = "get_hyperlink")]
    fn hyperlink(&self) -> Option<Hyperlink>;

    #[doc(alias = "atspi_accessible_get_hypertext_iface")]
    #[doc(alias = "get_hypertext_iface")]
    fn hypertext_iface(&self) -> Option<Hypertext>;

    #[doc(alias = "atspi_accessible_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_accessible_get_image_iface")]
    #[doc(alias = "get_image_iface")]
    fn image_iface(&self) -> Option<Image>;

    #[doc(alias = "atspi_accessible_get_index_in_parent")]
    #[doc(alias = "get_index_in_parent")]
    fn index_in_parent(&self) -> Result<i32, glib::Error>;

    //#[doc(alias = "atspi_accessible_get_interfaces")]
    //#[doc(alias = "get_interfaces")]
    //fn interfaces(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 };

    #[doc(alias = "atspi_accessible_get_localized_role_name")]
    #[doc(alias = "get_localized_role_name")]
    fn localized_role_name(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_object_locale")]
    #[doc(alias = "get_object_locale")]
    fn object_locale(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_parent")]
    #[doc(alias = "get_parent")]
    fn parent(&self) -> Result<Option<Accessible>, glib::Error>;

    #[doc(alias = "atspi_accessible_get_process_id")]
    #[doc(alias = "get_process_id")]
    fn process_id(&self) -> Result<(), glib::Error>;

    //#[doc(alias = "atspi_accessible_get_relation_set")]
    //#[doc(alias = "get_relation_set")]
    //fn relation_set(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 19 }, glib::Error>;

    #[doc(alias = "atspi_accessible_get_role")]
    #[doc(alias = "get_role")]
    fn role(&self) -> Result<Role, glib::Error>;

    #[doc(alias = "atspi_accessible_get_role_name")]
    #[doc(alias = "get_role_name")]
    fn role_name(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_selection_iface")]
    #[doc(alias = "get_selection_iface")]
    fn selection_iface(&self) -> Option<Selection>;

    #[doc(alias = "atspi_accessible_get_state_set")]
    #[doc(alias = "get_state_set")]
    fn state_set(&self) -> Option<StateSet>;

    #[doc(alias = "atspi_accessible_get_table_cell")]
    #[doc(alias = "get_table_cell")]
    fn table_cell(&self) -> Option<TableCell>;

    #[doc(alias = "atspi_accessible_get_table_iface")]
    #[doc(alias = "get_table_iface")]
    fn table_iface(&self) -> Option<Table>;

    #[doc(alias = "atspi_accessible_get_text_iface")]
    #[doc(alias = "get_text_iface")]
    fn text_iface(&self) -> Option<Text>;

    #[doc(alias = "atspi_accessible_get_toolkit_name")]
    #[doc(alias = "get_toolkit_name")]
    fn toolkit_name(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_toolkit_version")]
    #[doc(alias = "get_toolkit_version")]
    fn toolkit_version(&self) -> Result<glib::GString, glib::Error>;

    #[doc(alias = "atspi_accessible_get_value_iface")]
    #[doc(alias = "get_value_iface")]
    fn value_iface(&self) -> Option<Value>;

    #[doc(alias = "atspi_accessible_set_cache_mask")]
    fn set_cache_mask(&self, mask: Cache);

    #[doc(alias = "mode-changed")]
    fn connect_mode_changed<F: Fn(&Self, i32, &str) + 'static>(&self, detail: Option<&str>, f: F) -> SignalHandlerId;

    #[doc(alias = "region-changed")]
    fn connect_region_changed<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Accessible>> AccessibleExt for O {
    fn clear_cache(&self) {
        unsafe {
            ffi::atspi_accessible_clear_cache(self.as_ref().to_glib_none().0);
        }
    }

    fn accessible_id(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_accessible_id(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn action_iface(&self) -> Option<Action> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_action_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn application(&self) -> Result<Accessible, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_application(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn atspi_version(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_atspi_version(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //fn attributes(&self) -> Result</*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }, glib::Error> {
    //    unsafe { TODO: call ffi:atspi_accessible_get_attributes() }
    //}

    //fn attributes_as_array(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }, glib::Error> {
    //    unsafe { TODO: call ffi:atspi_accessible_get_attributes_as_array() }
    //}

    fn child_at_index(&self, child_index: i32) -> Result<Accessible, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_child_at_index(self.as_ref().to_glib_none().0, child_index, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn child_count(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_child_count(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn collection_iface(&self) -> Option<Collection> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_collection_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn component_iface(&self) -> Option<Component> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_component_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn description(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_description(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn document_iface(&self) -> Option<Document> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_document_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn editable_text_iface(&self) -> Option<EditableText> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_editable_text_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn hyperlink(&self) -> Option<Hyperlink> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_hyperlink(self.as_ref().to_glib_none().0))
        }
    }

    fn hypertext_iface(&self) -> Option<Hypertext> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_hypertext_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn id(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_id(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn image_iface(&self) -> Option<Image> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_image_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn index_in_parent(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_index_in_parent(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //fn interfaces(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call ffi:atspi_accessible_get_interfaces() }
    //}

    fn localized_role_name(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_localized_role_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn name(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn object_locale(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_object_locale(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn parent(&self) -> Result<Option<Accessible>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_parent(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn process_id(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_accessible_get_process_id(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn relation_set(&self) -> Result</*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 19 }, glib::Error> {
    //    unsafe { TODO: call ffi:atspi_accessible_get_relation_set() }
    //}

    fn role(&self) -> Result<Role, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_role(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn role_name(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_role_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn selection_iface(&self) -> Option<Selection> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_selection_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn state_set(&self) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_state_set(self.as_ref().to_glib_none().0))
        }
    }

    fn table_cell(&self) -> Option<TableCell> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_table_cell(self.as_ref().to_glib_none().0))
        }
    }

    fn table_iface(&self) -> Option<Table> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_table_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn text_iface(&self) -> Option<Text> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_text_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn toolkit_name(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_toolkit_name(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn toolkit_version(&self) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_accessible_get_toolkit_version(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn value_iface(&self) -> Option<Value> {
        unsafe {
            from_glib_full(ffi::atspi_accessible_get_value_iface(self.as_ref().to_glib_none().0))
        }
    }

    fn set_cache_mask(&self, mask: Cache) {
        unsafe {
            ffi::atspi_accessible_set_cache_mask(self.as_ref().to_glib_none().0, mask.into_glib());
        }
    }

    fn connect_mode_changed<F: Fn(&Self, i32, &str) + 'static>(&self, detail: Option<&str>, f: F) -> SignalHandlerId {
        unsafe extern "C" fn mode_changed_trampoline<P: IsA<Accessible>, F: Fn(&P, i32, &str) + 'static>(this: *mut ffi::AtspiAccessible, arg1: libc::c_int, why: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Accessible::from_glib_borrow(this).unsafe_cast_ref(), arg1, &glib::GString::from_glib_borrow(why))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| { format!("mode-changed::{}\0", name) });
            let signal_name: &[u8] = detailed_signal_name.as_ref().map_or(&b"mode-changed\0"[..], |n| n.as_bytes());
            connect_raw(self.as_ptr() as *mut _, signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(mode_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_region_changed<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn region_changed_trampoline<P: IsA<Accessible>, F: Fn(&P, i32, i32) + 'static>(this: *mut ffi::AtspiAccessible, arg1: libc::c_int, arg2: libc::c_int, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Accessible::from_glib_borrow(this).unsafe_cast_ref(), arg1, arg2)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"region-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(region_changed_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Accessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Accessible")
    }
}