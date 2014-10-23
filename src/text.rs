use libc;
use std;
use super::actor::Actor;
use super::content::Content;

#[repr(C)]
pub struct TextRef {
  opaque: *mut libc::c_void
}

impl TextRef {
  pub fn new() -> TextRef {
    unsafe {
      let foreign_result = clutter_text_new();
      return foreign_result;
    }
  }

  pub fn new_full(font_name: &str, text: &str, color: super::color::Color) -> TextRef {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_new_full(font_name.to_c_str().unwrap() as *mut i8, text.to_c_str().unwrap() as *mut i8, color);
      return foreign_result;
    }
  }

  pub fn new_with_text(font_name: &str, text: &str) -> TextRef {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_new_with_text(font_name.to_c_str().unwrap() as *mut i8, text.to_c_str().unwrap() as *mut i8);
      return foreign_result;
    }
  }

  pub fn new_with_buffer<T: Buffer>(buffer: &T) -> TextRef {
    unsafe {
      let foreign_result = clutter_text_new_with_buffer(buffer.as_buffer());
      return foreign_result;
    }
  }
}

pub trait Text {
  fn as_text(&self) -> *mut libc::c_void;

  fn set_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  fn get_color(&mut self) -> super::color::Color {
    unsafe {
      let foreign_result = clutter_text_get_color(self.as_text());
      return foreign_result;
    }
  }

  fn set_font_name(&mut self, text: &str) {
    unsafe {
      use std::c_str::ToCStr;
      clutter_text_set_font_name(self.as_text(), text.to_c_str().unwrap() as *mut i8);
    }
  }

  fn get_font_name(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_text_get_font_name(self.as_text());
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  fn set_line_wrap(&mut self, fixed_position_set: bool) {
    unsafe {
      clutter_text_set_line_wrap(self.as_text(), (fixed_position_set as i32));
    }
  }

  fn get_line_wrap(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_text_get_line_wrap(self.as_text());
      return foreign_result != 0;
    }
  }

  fn set_selectable(&mut self, fixed_position_set: bool) {
    unsafe {
      clutter_text_set_selectable(self.as_text(), (fixed_position_set as i32));
    }
  }

  fn get_selectable(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_text_get_selectable(self.as_text());
      return foreign_result != 0;
    }
  }

  fn set_editable(&mut self, fixed_position_set: bool) {
    unsafe {
      clutter_text_set_editable(self.as_text(), (fixed_position_set as i32));
    }
  }

  fn get_editable(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_text_get_editable(self.as_text());
      return foreign_result != 0;
    }
  }

  fn set_cursor_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_cursor_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  fn get_cursor_color(&mut self) -> super::color::Color {
    unsafe {
      let foreign_result = clutter_text_get_cursor_color(self.as_text());
      return foreign_result;
    }
  }

  fn set_selection_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_selection_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  fn get_selection_color(&mut self) -> super::color::Color {
    unsafe {
      let foreign_result = clutter_text_get_selection_color(self.as_text());
      return foreign_result;
    }
  }

  fn set_selected_text_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_selected_text_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  fn get_selected_text_color(&mut self) -> super::color::Color {
    unsafe {
      let foreign_result = clutter_text_get_selected_text_color(self.as_text());
      return foreign_result;
    }
  }
}

impl Text for TextRef {
  fn as_text(&self) -> *mut libc::c_void {
    return self.opaque;
  }
}

impl Actor for TextRef {
  fn as_actor(&self) -> *mut libc::c_void {
    return self.opaque;
  }
}

impl Content for TextRef {
  fn as_content(&self) -> *mut libc::c_void {
    return self.opaque;
  }
}

extern {
  fn clutter_text_new() -> TextRef;
  fn clutter_text_new_full(font_name: *mut libc::c_char, text: *mut libc::c_char, color: super::color::Color) -> TextRef;
  fn clutter_text_new_with_text(font_name: *mut libc::c_char, text: *mut libc::c_char) -> TextRef;
  fn clutter_text_new_with_buffer(buffer: *mut libc::c_void) -> TextRef;
  fn clutter_text_set_color(self_value: *mut libc::c_void, color: *mut libc::c_void);
  fn clutter_text_get_color(self_value: *mut libc::c_void) -> super::color::Color;
  fn clutter_text_set_font_name(self_value: *mut libc::c_void, text: *mut libc::c_char);
  fn clutter_text_get_font_name(self_value: *mut libc::c_void) -> *mut i8;
  fn clutter_text_set_line_wrap(self_value: *mut libc::c_void, fixed_position_set: i32);
  fn clutter_text_get_line_wrap(self_value: *mut libc::c_void) -> i32;
  fn clutter_text_set_selectable(self_value: *mut libc::c_void, fixed_position_set: i32);
  fn clutter_text_get_selectable(self_value: *mut libc::c_void) -> i32;
  fn clutter_text_set_editable(self_value: *mut libc::c_void, fixed_position_set: i32);
  fn clutter_text_get_editable(self_value: *mut libc::c_void) -> i32;
  fn clutter_text_set_cursor_color(self_value: *mut libc::c_void, color: *mut libc::c_void);
  fn clutter_text_get_cursor_color(self_value: *mut libc::c_void) -> super::color::Color;
  fn clutter_text_set_selection_color(self_value: *mut libc::c_void, color: *mut libc::c_void);
  fn clutter_text_get_selection_color(self_value: *mut libc::c_void) -> super::color::Color;
  fn clutter_text_set_selected_text_color(self_value: *mut libc::c_void, color: *mut libc::c_void);
  fn clutter_text_get_selected_text_color(self_value: *mut libc::c_void) -> super::color::Color;
}

#[repr(C)]
pub struct BufferRef {
  opaque: *mut libc::c_void
}

impl BufferRef {
  pub fn new() -> BufferRef {
    unsafe {
      let foreign_result = clutter_text_buffer_new();
      return foreign_result;
    }
  }

  pub fn new_with_text(text: &str) -> BufferRef {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_buffer_new_with_text(text.to_c_str().unwrap() as *mut i8, -1);
      return foreign_result;
    }
  }
}

pub trait Buffer {
  fn as_buffer(&self) -> *mut libc::c_void;

  fn set_text(&mut self, text: &str) {
    unsafe {
      use std::c_str::ToCStr;
      clutter_text_buffer_set_text(self.as_buffer(), text.to_c_str().unwrap() as *mut i8, -1);
    }
  }

  fn get_text(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_text_buffer_get_text(self.as_buffer());
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  fn get_bytes(&mut self) -> u32 {
    unsafe {
      let foreign_result = clutter_text_buffer_get_bytes(self.as_buffer());
      return foreign_result;
    }
  }

  fn get_length(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_text_buffer_get_length(self.as_buffer());
      return foreign_result;
    }
  }

  fn set_max_length(&mut self, max_length: i32) {
    unsafe {
      clutter_text_buffer_set_max_length(self.as_buffer(), max_length);
    }
  }

  fn get_max_length(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_text_buffer_get_max_length(self.as_buffer());
      return foreign_result;
    }
  }

  fn insert_text(&mut self, position: i32, chars: &str) -> i32 {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_buffer_insert_text(self.as_buffer(), position, chars.to_c_str().unwrap() as *mut i8, -1);
      return foreign_result;
    }
  }

  fn delete_text(&mut self, position: i32, n_chars: i32) -> i32 {
    unsafe {
      let foreign_result = clutter_text_buffer_delete_text(self.as_buffer(), position, n_chars);
      return foreign_result;
    }
  }

  fn emit_inserted_text(&mut self, position: i32, chars: &str) -> i32 {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_buffer_emit_inserted_text(self.as_buffer(), position, chars.to_c_str().unwrap() as *mut i8, -1);
      return foreign_result;
    }
  }

  fn emit_deleted_text(&mut self, position: i32, n_chars: i32) -> i32 {
    unsafe {
      let foreign_result = clutter_text_buffer_emit_deleted_text(self.as_buffer(), position, n_chars);
      return foreign_result;
    }
  }
}

impl Buffer for BufferRef {
  fn as_buffer(&self) -> *mut libc::c_void {
    return self.opaque;
  }
}

extern {
  fn clutter_text_buffer_new() -> BufferRef;
  fn clutter_text_buffer_new_with_text(text: *mut libc::c_char, utf8_len: i32) -> BufferRef;
  fn clutter_text_buffer_set_text(self_value: *mut libc::c_void, text: *mut libc::c_char, utf8_len: i32);
  fn clutter_text_buffer_get_text(self_value: *mut libc::c_void) -> *mut i8;
  fn clutter_text_buffer_get_bytes(self_value: *mut libc::c_void) -> libc::c_uint;
  fn clutter_text_buffer_get_length(self_value: *mut libc::c_void) -> i32;
  fn clutter_text_buffer_set_max_length(self_value: *mut libc::c_void, max_length: i32);
  fn clutter_text_buffer_get_max_length(self_value: *mut libc::c_void) -> i32;
  fn clutter_text_buffer_insert_text(self_value: *mut libc::c_void, position: i32, chars: *mut libc::c_char, n_chars: i32) -> i32;
  fn clutter_text_buffer_delete_text(self_value: *mut libc::c_void, position: i32, n_chars: i32) -> i32;
  fn clutter_text_buffer_emit_inserted_text(self_value: *mut libc::c_void, position: i32, chars: *mut libc::c_char, n_chars: i32) -> i32;
  fn clutter_text_buffer_emit_deleted_text(self_value: *mut libc::c_void, position: i32, n_chars: i32) -> i32;
}

