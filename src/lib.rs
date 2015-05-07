// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(raw_pointer_derive)]

extern crate libc;

#[macro_use]
mod link;
mod internal;

#[cfg(feature="glx")]
pub mod glx;
pub mod keysym;
#[cfg(feature="xcursor")]
pub mod xcursor;
#[cfg(feature="xf86vmode")]
pub mod xf86vmode;
#[cfg(feature="xlib")]
pub mod xlib;
#[cfg(feature="xrender")]
pub mod xrender;
