#![stable]

use libc;
use std;
use cairo;
use super::content::Content;

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct CanvasRef {
  opaque: *mut libc::c_void
}

impl CanvasRef {
  /// Creates a new instance of Canvas.
  ///
  /// You should call `.set_size()` to set the size of the canvas.
  ///
  /// You should call `.invalidate()` every time you wish to draw the contents
  /// of the canvas.
  ///
  /// _Since 1.10_
  pub fn new() -> CanvasRef {
    unsafe {
      let foreign_result = clutter_canvas_new();
      return foreign_result;
    }
  }
}

/// Content for 2D painting
///
/// The Canvas class is a Content implementation that allows drawing using the
/// Cairo API on a 2D surface.
///
/// In order to draw on a Canvas, you should connect a handler to the `draw`
/// signal; the signal will receive a cairo context that can be used to draw.
/// Canvas will emit the `draw` signal when invalidated using `.invalidate()`.
///
/// _Since 1.10_
pub trait Canvas {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
  fn as_canvas(&self) -> *mut libc::c_void;

  /// Sets the size of the canvas, and invalidates the content.
  ///
  /// This method will cause the canvas to be invalidated only if the size of
  /// the canvas surface has changed.
  ///
  /// If you want to invalidate the contents of the canvas when setting the
  /// size, you can use the return value of the method to conditionally call
  /// `.invalidate()`.
  ///
  /// _Since 1.10_
  fn set_size(&mut self, width: i32, height: i32) {
    unsafe {
      clutter_canvas_set_size(self.as_canvas(), width, height);
    }
  }

  //FIXME: doc
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

//FIXME: doc
extern "C" fn handler_for_on_draw(canvas: *mut libc::c_void, cairo: *mut libc::c_void, width: i32, height: i32, handler: *mut libc::c_void) -> i32 {
  unsafe {
    let mut canvas_r = CanvasRef { opaque: canvas };
    let mut cairo_r = cairo::Cairo { opaque: cairo };
    let handler = std::mem::transmute::<*mut libc::c_void, &mut |canvas: &mut CanvasRef, cairo: &mut cairo::Cairo, width: i32, height: i32| -> bool>(handler);
    let foreign_result = (*handler)(&mut canvas_r, &mut cairo_r, width, height);
    std::mem::forget(canvas_r);
    std::mem::forget(cairo_r);
    return foreign_result as i32;
  }
}

extern {
  fn clutter_canvas_new() -> CanvasRef;
  #[link_name = "g_signal_connect_data"]
  fn rsi_connect_on_draw(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut libc::c_void, i32, i32, *mut libc::c_void) -> i32, data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
  fn clutter_canvas_set_size(self_value: *mut libc::c_void, width: i32, height: i32);
}

