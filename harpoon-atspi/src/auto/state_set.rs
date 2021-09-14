// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 4b6cabc)
// DO NOT EDIT

use crate::StateType;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtspiStateSet")]
    pub struct StateSet(Object<ffi::AtspiStateSet, ffi::AtspiStateSetClass>);

    match fn {
        type_ => || ffi::atspi_state_set_get_type(),
    }
}

impl StateSet {
    //#[doc(alias = "atspi_state_set_new")]
    //pub fn new(states: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 69 }) -> StateSet {
    //    unsafe { TODO: call ffi:atspi_state_set_new() }
    //}
}

pub const NONE_STATE_SET: Option<&StateSet> = None;

pub trait StateSetExt: 'static {
    #[doc(alias = "atspi_state_set_add")]
    fn add(&self, state: StateType);

    #[doc(alias = "atspi_state_set_compare")]
    fn compare<P: IsA<StateSet>>(&self, set2: &P) -> Option<StateSet>;

    #[doc(alias = "atspi_state_set_contains")]
    fn contains(&self, state: StateType) -> bool;

    #[doc(alias = "atspi_state_set_equals")]
    fn equals<P: IsA<StateSet>>(&self, set2: &P) -> bool;

    //#[doc(alias = "atspi_state_set_get_states")]
    //#[doc(alias = "get_states")]
    //fn states(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 69 };

    #[doc(alias = "atspi_state_set_is_empty")]
    fn is_empty(&self) -> bool;

    #[doc(alias = "atspi_state_set_remove")]
    fn remove(&self, state: StateType);

    #[doc(alias = "atspi_state_set_set_by_name")]
    fn set_by_name(&self, name: &str, enabled: bool);
}

impl<O: IsA<StateSet>> StateSetExt for O {
    fn add(&self, state: StateType) {
        unsafe {
            ffi::atspi_state_set_add(self.as_ref().to_glib_none().0, state.into_glib());
        }
    }

    fn compare<P: IsA<StateSet>>(&self, set2: &P) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atspi_state_set_compare(self.as_ref().to_glib_none().0, set2.as_ref().to_glib_none().0))
        }
    }

    fn contains(&self, state: StateType) -> bool {
        unsafe {
            from_glib(ffi::atspi_state_set_contains(self.as_ref().to_glib_none().0, state.into_glib()))
        }
    }

    fn equals<P: IsA<StateSet>>(&self, set2: &P) -> bool {
        unsafe {
            from_glib(ffi::atspi_state_set_equals(self.as_ref().to_glib_none().0, set2.as_ref().to_glib_none().0))
        }
    }

    //fn states(&self) -> /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 69 } {
    //    unsafe { TODO: call ffi:atspi_state_set_get_states() }
    //}

    fn is_empty(&self) -> bool {
        unsafe {
            from_glib(ffi::atspi_state_set_is_empty(self.as_ref().to_glib_none().0))
        }
    }

    fn remove(&self, state: StateType) {
        unsafe {
            ffi::atspi_state_set_remove(self.as_ref().to_glib_none().0, state.into_glib());
        }
    }

    fn set_by_name(&self, name: &str, enabled: bool) {
        unsafe {
            ffi::atspi_state_set_set_by_name(self.as_ref().to_glib_none().0, name.to_glib_none().0, enabled.into_glib());
        }
    }
}

impl fmt::Display for StateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StateSet")
    }
}