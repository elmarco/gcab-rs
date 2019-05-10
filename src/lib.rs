extern crate gcab_sys;
#[macro_use]
extern crate glib;
extern crate glib_sys;

extern crate gio;
extern crate gobject_sys;

extern crate libc;
#[macro_use]
extern crate bitflags;

pub use auto::*;

mod auto;

pub use glib::{Error, Object};
