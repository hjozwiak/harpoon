// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 520ff74)
// DO NOT EDIT

use crate::RelationType;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtspiRelation")]
    pub struct Relation(Object<ffi::AtspiRelation, ffi::AtspiRelationClass>);

    match fn {
        type_ => || ffi::atspi_relation_get_type(),
    }
}

pub const NONE_RELATION: Option<&Relation> = None;

pub trait RelationExt: 'static {
    #[doc(alias = "atspi_relation_get_n_targets")]
    #[doc(alias = "get_n_targets")]
    fn n_targets(&self) -> i32;

    #[doc(alias = "atspi_relation_get_relation_type")]
    #[doc(alias = "get_relation_type")]
    fn relation_type(&self) -> RelationType;

    //#[doc(alias = "atspi_relation_get_target")]
    //#[doc(alias = "get_target")]
    //fn target(&self, i: i32) -> /*Ignored*/Option<Accessible>;
}

impl<O: IsA<Relation>> RelationExt for O {
    fn n_targets(&self) -> i32 {
        unsafe {
            ffi::atspi_relation_get_n_targets(self.as_ref().to_glib_none().0)
        }
    }

    fn relation_type(&self) -> RelationType {
        unsafe {
            from_glib(ffi::atspi_relation_get_relation_type(self.as_ref().to_glib_none().0))
        }
    }

    //fn target(&self, i: i32) -> /*Ignored*/Option<Accessible> {
    //    unsafe { TODO: call ffi:atspi_relation_get_target() }
    //}
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Relation")
    }
}
