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

pub fn init() {
  unsafe {
    clutter_init(std::ptr::null_mut(), std::ptr::null_mut());
  }
}

pub fn main() {
  unsafe {
    clutter_main();
  }
}

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
