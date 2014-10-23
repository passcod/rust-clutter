use libc;
use std;

#[repr(C)]
pub struct ContentRef {
  opaque: *mut libc::c_void
}

pub trait Content {
  fn as_content(&self) -> *mut libc::c_void;

  fn get_preferred_size(&mut self) -> (bool, f32, f32) {
    unsafe {
      let mut width:f32 = std::intrinsics::init();
      let mut height:f32 = std::intrinsics::init();
      let foreign_result = clutter_content_get_preferred_size(self.as_content(), &mut width, &mut height);
      return (foreign_result != 0, width, height);
    }
  }

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

