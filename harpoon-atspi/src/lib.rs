#[macro_use]
extern crate glib;
extern crate glib_sys;
extern crate ffi;

extern crate libc;
#[macro_use]
extern crate bitflags;


pub use auto::*;
mod auto;
mod rect;
pub use rect::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
