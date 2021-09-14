#[macro_use]
mod rt;
#[macro_use]
extern crate glib;
extern crate ffi;
extern crate glib_sys;
extern crate libc;
#[macro_use]
extern crate bitflags;

pub use auto::*;
mod auto;
