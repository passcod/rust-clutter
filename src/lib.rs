//! Clutter is a GObject based library for creating fast, visually rich,
//! graphical user interfaces.
//!
//! ---------------------------------------------------------------------------
//!
//! This library is free software; you can redistribute it and/or modify it
//! under the terms of the GNU Library General Public License as published by
//! the Free Software Foundation; either version 2 of the License, or (at your
//! option) any later version.
//!
//! This library is distributed in the hope that it will be useful, but WITHOUT
//! ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//! FITNESS FOR A PARTICULAR PURPOSE. See the GNU Library General Public
//! License for more details.
//!
//! You may obtain a copy of the GNU Library General Public License from the
//! Free Software Foundation by visiting their Web site or by writing to:
//!
//! Free Software Foundation, Inc.  
//! 59 Temple Place - Suite 330  
//! Boston, MA 02111-1307  
//! USA  
//!
//! ---------------------------------------------------------------------------
//!
//! Permission is granted to copy, distribute and/or modify this documentation
//! under the terms of the GNU Free Documentation License, Version 1.1 or any
//! later version published by the Free Software Foundation with no Invariant
//! Sections, no Front-Cover Texts, and no Back-Cover Texts. You may obtain a
//! copy of the GNU Free Documentation License from the Free Software
//! Foundation by visiting their Web site or by writing to the address above.
#![stable]

extern crate cairo;
extern crate libc;

#[link(name = "gobject-2.0")] extern {}
#[link(name = "clutter-1.0")] extern {}

pub mod actor;
pub mod canvas;
pub mod color;
pub mod constraint;
pub mod content;
pub mod scaling;
pub mod stage;
pub mod text;

/// Initialises everything needed to operate with Clutter.
///
/// It is safe to call this function multiple times.
///
/// This function will not abort in case of errors during initialization;
/// `init()` will print out the error message on stderr, and will return an
/// error code. It is up to the application code to handle this case.
pub fn init() {
  unsafe {
    clutter_init(std::ptr::null_mut(), std::ptr::null_mut());
  }
}

/// Starts the Clutter mainloop.
pub fn main() {
  unsafe {
    clutter_main();
  }
}

/// Terminates the Clutter mainloop.
pub fn main_quit() {
  unsafe {
    clutter_main_quit();
  }
}

extern {
  fn clutter_init(argc: *mut i32, argv: *mut i32);
  fn clutter_main();
  fn clutter_main_quit();
}
