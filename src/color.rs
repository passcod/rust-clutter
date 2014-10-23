use libc;
use std;

#[repr(C)]
pub struct Color {
  pub opaque: *mut libc::c_void
}

impl Color {
  pub fn new(red: i8, green: i8, blue: i8, alpha: i8) -> Color {
    unsafe {
      let foreign_result = clutter_color_new(red, green, blue, alpha);
      return foreign_result;
    }
  }

  pub fn alloc() -> Color {
    unsafe {
      let foreign_result = clutter_color_alloc();
      return foreign_result;
    }
  }

  pub fn init(&mut self, red: i8, green: i8, blue: i8, alpha: i8) {
    unsafe {
      clutter_color_init(self.opaque, red, green, blue, alpha);
    }
  }

  pub fn equal(&self, other: &Color) -> bool {
    unsafe {
      let foreign_result = clutter_color_equal(self.opaque as *mut libc::c_void, other.opaque as *mut libc::c_void);
      return foreign_result != 0;
    }
  }

  pub fn hash(&self) -> i32 {
    unsafe {
      let foreign_result = clutter_color_hash(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  pub fn from_string(&mut self, name: &str) -> bool {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_color_from_string(self.opaque, name.to_c_str().unwrap() as *mut i8);
      return foreign_result != 0;
    }
  }

  pub fn to_string(&self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_color_to_string(self.opaque as *mut libc::c_void);
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  pub fn from_hls(&mut self, hue: f32, luminance: f32, saturation: f32) {
    unsafe {
      clutter_color_from_hls(self.opaque, hue, luminance, saturation);
    }
  }

  pub fn to_hls(&self) -> (f32, f32, f32) {
    unsafe {
      let mut hue:f32 =std::intrinsics::init();
      let mut luminance:f32 =std::intrinsics::init();
      let mut saturation:f32 =std::intrinsics::init();
      clutter_color_to_hls(self.opaque as *mut libc::c_void, &mut hue, &mut luminance, &mut saturation);
      return (hue, luminance, saturation);
    }
  }

  pub fn from_pixel(&mut self, pixel: i32) {
    unsafe {
      clutter_color_from_pixel(self.opaque, pixel);
    }
  }

  pub fn to_pixel(&self) -> i32 {
    unsafe {
      let foreign_result = clutter_color_to_pixel(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  pub fn add(&self, other: &Color) -> Color {
    unsafe {
      let foreign_result = clutter_color_add(self.opaque as *mut libc::c_void, other.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  pub fn subtract(&self, other: &Color) -> Color {
    unsafe {
      let foreign_result = clutter_color_subtract(self.opaque as *mut libc::c_void, other.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  pub fn lighten(&self) -> Color {
    unsafe {
      let foreign_result = clutter_color_lighten(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  pub fn darken(&self) -> Color {
    unsafe {
      let foreign_result = clutter_color_darken(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  pub fn shade(&self, factor: f64) -> Color {
    unsafe {
      let foreign_result = clutter_color_shade(self.opaque as *mut libc::c_void, factor);
      return foreign_result;
    }
  }

  pub fn interpolate(&self, final_color: &Color, progress: f64) -> Color {
    unsafe {
      let foreign_result = clutter_color_interpolate(self.opaque as *mut libc::c_void, final_color.opaque as *mut libc::c_void, progress);
      return foreign_result;
    }
  }
}

extern {
  fn clutter_color_new(red: i8, green: i8, blue: i8, alpha: i8) -> Color;
  fn clutter_color_alloc() -> Color;
  fn clutter_color_init(self_value: *mut libc::c_void, red: i8, green: i8, blue: i8, alpha: i8);
  fn clutter_color_equal(self_value: *mut libc::c_void, other: *mut libc::c_void) -> i32;
  fn clutter_color_hash(self_value: *mut libc::c_void) -> i32;
  fn clutter_color_from_string(self_value: *mut libc::c_void, name: *mut libc::c_char) -> i32;
  fn clutter_color_to_string(self_value: *mut libc::c_void) -> *mut i8;
  fn clutter_color_from_hls(self_value: *mut libc::c_void, hue: f32, luminance: f32, saturation: f32);
  fn clutter_color_to_hls(self_value: *mut libc::c_void, hue: *mut f32, luminance: *mut f32, saturation: *mut f32);
  fn clutter_color_from_pixel(self_value: *mut libc::c_void, pixel: i32);
  fn clutter_color_to_pixel(self_value: *mut libc::c_void) -> i32;
  fn clutter_color_add(self_value: *mut libc::c_void, other: *mut libc::c_void) -> Color;
  fn clutter_color_subtract(self_value: *mut libc::c_void, other: *mut libc::c_void) -> Color;
  fn clutter_color_lighten(self_value: *mut libc::c_void) -> Color;
  fn clutter_color_darken(self_value: *mut libc::c_void) -> Color;
  fn clutter_color_shade(self_value: *mut libc::c_void, factor: f64) -> Color;
  fn clutter_color_interpolate(self_value: *mut libc::c_void, final_color: *mut libc::c_void, progress: f64) -> Color;
}

impl std::clone::Clone for Color {
  fn clone(&self) -> Color {
    unsafe {
      let foreign_result = clutter_color_copy(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }
}

extern {
  fn clutter_color_copy(self_value: *mut libc::c_void) -> Color;
}

//FIXME: What was DeepClone and is it still needed?
/*impl DeepClone for Color {
  fn deep_clone(&self) -> Color {
    unsafe {
      let foreign_result = clutter_color_copy(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }
}*/

impl std::ops::Drop for Color {
  fn drop(&mut self) {
    unsafe {
      clutter_color_free(self.opaque);
    }
  }
}

extern {
  fn clutter_color_free(self_value: *mut libc::c_void);
}

