#![stable]

use libc;
use std;

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct ContentRef {
  opaque: *mut libc::c_void
}

/// Delegate for painting the content of an actor
///
/// Content is an interface to implement types responsible for painting the
/// content of an Actor.
///
/// Multiple actors can use the same Content instance, in order to share the
/// resources associated with painting the same content.
///
/// _Since 1.10_
pub trait Content {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
  fn as_content(&self) -> *mut libc::c_void;

  /// Retrieves the natural size of the content, if any.
  ///
  /// The natural size of a Content is defined as the size the content would
  /// have regardless of the allocation of the actor that is painting it, for
  /// instance the size of an image data.
  ///
  /// _Since 1.10_
  fn get_preferred_size(&mut self) -> (bool, f32, f32) {
    unsafe {
      let mut width:f32 = std::intrinsics::init();
      let mut height:f32 = std::intrinsics::init();
      let foreign_result = clutter_content_get_preferred_size(self.as_content(), &mut width, &mut height);
      return (foreign_result != 0, width, height);
    }
  }

  /// Invalidates a Content.
  ///
  /// This function should be called by Content implementations when they change
  /// the way the content should be painted regardless of the actor state.
  ///
  /// _Since 1.10_
  fn invalidate(&mut self) {
    unsafe {
      clutter_content_invalidate(self.as_content());
    }
  }
}

impl Content for ContentRef {
  fn as_content(&self) -> *mut libc::c_void {
    return self.opaque;
  }
}

extern {
  fn clutter_content_get_preferred_size(self_value: *mut libc::c_void, width: *mut f32, height: *mut f32) -> i32;
  fn clutter_content_invalidate(self_value: *mut libc::c_void);
}
