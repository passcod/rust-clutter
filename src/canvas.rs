use libc;
use std;
use cairo;
use super::content::Content;

pub struct CanvasRef {
  opaque: *mut libc::c_void
}

impl CanvasRef {
  pub fn new() -> CanvasRef {
    unsafe {
      let foreign_result = clutter_canvas_new();
      return foreign_result;
    }
  }
}

pub trait Canvas {
  fn as_canvas(&self) -> *mut libc::c_void;

  fn set_size(&mut self, width: i32, height: i32) {
    unsafe {
      clutter_canvas_set_size(self.as_canvas(), width, height);
    }
  }

  fn on_draw(&mut self, handler: &|&mut CanvasRef, &mut cairo::Cairo, i32, i32| -> bool) -> u64 {
    unsafe {
      let null_void: *mut libc::c_void = std::ptr::null_mut();
      return rsi_connect_on_draw(self.as_canvas(), "draw".to_c_str().unwrap() as *mut i8, handler_for_on_draw, std::mem::transmute::<&|&mut CanvasRef, &mut cairo::Cairo, i32, i32| -> bool, *mut libc::c_void>(handler), null_void, 0);
    }
  }
}

impl Canvas for CanvasRef {
  fn as_canvas(&self) -> *mut libc::c_void {
    return self.opaque;
  }
}

impl Content for CanvasRef {
  fn as_content(&self) -> *mut libc::c_void {
    return self.opaque;
  }
}

extern "C" fn handler_for_on_draw(canvas: *mut libc::c_void, cairo: *mut libc::c_void, width: i32, height: i32, handler: *mut libc::c_void) -> i32 {
  unsafe {
    let mut canvas_r = CanvasRef { opaque: canvas };
    let mut cairo_r = cairo::Cairo { opaque: cairo };
    let handler = std::mem::transmute::<*mut libc::c_void, &mut |canvas: &mut CanvasRef, cairo: &mut cairo::Cairo, width: i32, height: i32| -> bool>(handler);
    let foreign_result = (*handler)(&mut canvas_r, &mut cairo_r, width, height);
    std::mem::forget(canvas_r);
    std::mem::forget(cairo_r);
    return (foreign_result as i32);
  }
}

extern {
  fn clutter_canvas_new() -> CanvasRef;
  #[link_name = "g_signal_connect_data"]
  fn rsi_connect_on_draw(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut libc::c_void, i32, i32, *mut libc::c_void) -> i32, data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
  fn clutter_canvas_set_size(self_value: *mut libc::c_void, width: i32, height: i32);
}

