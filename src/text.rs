#![stable]

use libc;
use std;
use super::actor::Actor;
use super::content::Content;

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct TextRef {
  opaque: *mut libc::c_void
}

impl TextRef {
  /// Creates a new Text actor.
  ///
  /// This actor can be used to display and edit text.
  ///
  /// _Since 1.0_
  pub fn new() -> TextRef {
    unsafe {
      let foreign_result = clutter_text_new();
      return foreign_result;
    }
  }

  /// Creates a new Text actor, using `font_name` as the font description.
  /// 
  /// `text` will be used to set the contents of the actor; and `color` will
  /// be used as the color to render `text`.
  ///
  /// This function is equivalent to calling `TextRef::new()` followed by
  /// `.set_font_name()`, `.set_text()` and `.set_color()`.
  ///
  /// _Since 1.0_
  pub fn new_full(font_name: &str, text: &str, color: super::color::Color) -> TextRef {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_new_full(font_name.to_c_str().unwrap() as *mut i8, text.to_c_str().unwrap() as *mut i8, color);
      return foreign_result;
    }
  }

  /// Creates a new ClutterText actor, using `font_name` as the font description.
  ///
  /// `text` will be used to set the contents of the actor.
  ///
  /// This function is equivalent to calling `TextRef::new()` followed by
  /// `.set_font_name()`, and `.set_text()`.
  ///
  /// _Since 1.0_
  pub fn new_with_text(font_name: &str, text: &str) -> TextRef {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_new_with_text(font_name.to_c_str().unwrap() as *mut i8, text.to_c_str().unwrap() as *mut i8);
      return foreign_result;
    }
  }

  /// Creates a new entry with the specified text buffer.
  ///
  /// _Since 1.10_
  pub fn new_with_buffer<T: Buffer>(buffer: &T) -> TextRef {
    unsafe {
      let foreign_result = clutter_text_new_with_buffer(buffer.as_buffer());
      return foreign_result;
    }
  }
}

/// An actor for displaying and editing text.
///
/// Text is an actor that displays custom text using Pango as the text
/// rendering engine.
///
/// Text also allows inline editing of the text if the actor is set editable
/// using `.set_editable()`.
///
/// Selection using keyboard or pointers can be enabled using
/// `.set_selectable()`.
///
/// _Since 1.0_
pub trait Text {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
  fn as_text(&self) -> *mut libc::c_void;

  /// Sets the color of the contents of a Text actor.
  ///
  /// The overall opacity of the Text actor will be the result of the alpha
  /// value of `color` and the composited opacity of the actor itself on the
  /// scenegraph, as returned by `Actor#get_paint_opacity()`.
  ///
  /// _Since 1.0_
  fn set_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  /// Retrieves the text color as set by `.set_color()`.
  ///
  /// _Since 1.0_
  fn get_color(&mut self) -> super::color::Color {
    unsafe {
      let foreign_result = clutter_text_get_color(self.as_text());
      return foreign_result;
    }
  }

  /// Sets the font used by a Text.
  ///
  /// The `font_name` string must be something that can be parsed by the
  /// `pango_font_description_from_string()` function.
  ///
  /// _Since 1.0_
  fn set_font_name(&mut self, text: &str) {
    unsafe {
      use std::c_str::ToCStr;
      clutter_text_set_font_name(self.as_text(), text.to_c_str().unwrap() as *mut i8);
    }
  }

  /// Retrieves the font name as set by `.set_font_name()`.
  ///
  /// _Since 1.0_
  fn get_font_name(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_text_get_font_name(self.as_text());
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  /// Sets whether the contents of a Text actor should wrap, if they don't fit
  /// the size assigned to the actor.
  ///
  /// _Since 1.0_
  fn set_line_wrap(&mut self, fixed_position_set: bool) {
    unsafe {
      clutter_text_set_line_wrap(self.as_text(), (fixed_position_set as i32));
    }
  }

  /// Retrieves the value set using `.set_line_wrap()`.
  fn get_line_wrap(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_text_get_line_wrap(self.as_text());
      return foreign_result != 0;
    }
  }

  /// Sets whether a Text actor should be selectable.
  ///
  /// A selectable Text will allow selecting its contents using the pointer or
  /// the keyboard.
  ///
  /// _Since 1.0_
  fn set_selectable(&mut self, fixed_position_set: bool) {
    unsafe {
      clutter_text_set_selectable(self.as_text(), (fixed_position_set as i32));
    }
  }

  /// Retrieves whether a Text is selectable or not.
  ///
  /// _Since 1.0_
  fn get_selectable(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_text_get_selectable(self.as_text());
      return foreign_result != 0;
    }
  }

  /// Sets whether the Text actor should be editable.
  ///
  /// An editable Text with key focus set using `.grab_key_focus()` or
  /// `Stage#set_key_focus()` will receive key events and will update its
  /// contents accordingly.
  ///
  /// _Since 1.0_
  fn set_editable(&mut self, fixed_position_set: bool) {
    unsafe {
      clutter_text_set_editable(self.as_text(), (fixed_position_set as i32));
    }
  }

  /// Retrieves whether a ClutterText is editable or not.
  ///
  /// _Since 1.0_
  fn get_editable(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_text_get_editable(self.as_text());
      return foreign_result != 0;
    }
  }

  /// Sets the color of the cursor of a Text actor.
  ///
  /// _Since 1.0_
  fn set_cursor_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_cursor_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  /// Retrieves the color of the cursor of a Text actor.
  ///
  /// _Since 1.0_
  fn get_cursor_color(&mut self) -> super::color::Color {
    unsafe {
      let foreign_result = clutter_text_get_cursor_color(self.as_text());
      return foreign_result;
    }
  }

  /// Sets the color of the selection of a Text actor.
  ///
  /// _Since 1.0_
  fn set_selection_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_selection_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  /// Retrieves the color of the selection of a Text actor.
  ///
  /// _Since 1.0_
  fn get_selection_color(&mut self) -> super::color::Color {
    unsafe {
      let foreign_result = clutter_text_get_selection_color(self.as_text());
      return foreign_result;
    }
  }

  /// Sets the selected text color of a Text actor.
  ///
  /// _Since 1.8_
  fn set_selected_text_color(&mut self, color: &super::color::Color) {
    unsafe {
      clutter_text_set_selected_text_color(self.as_text(), color.opaque as *mut libc::c_void);
    }
  }

  /// Retrieves the color of selected text of a Text actor.
  ///
  /// _Since 1.8_
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

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct BufferRef {
  opaque: *mut libc::c_void
}

impl BufferRef {
  /// Create a new Buffer object.
  ///
  /// _Since 1.10_
  pub fn new() -> BufferRef {
    unsafe {
      let foreign_result = clutter_text_buffer_new();
      return foreign_result;
    }
  }

  /// Create a new Buffer object with some text.
  ///
  /// _Since 1.10_
  pub fn new_with_text(text: &str) -> BufferRef {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_buffer_new_with_text(text.to_c_str().unwrap() as *mut i8, -1);
      return foreign_result;
    }
  }
}

/// Text buffer for Text.
///
/// The Buffer contains the actual text displayed in a Text widget.
///
/// A single Buffer object can be shared by multiple Text widgets which will
/// then share the same text content, but not the cursor position, visibility
/// attributes, icon etc.
///
/// _Since 1.10_
pub trait Buffer {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
  fn as_buffer(&self) -> *mut libc::c_void;

  /// Sets the text in the buffer.
  ///
  /// This is roughly equivalent to calling `.delete_text()` and
  /// `.insert_text()`.
  ///
  /// _Since 1.10_
  fn set_text(&mut self, text: &str) {
    unsafe {
      use std::c_str::ToCStr;
      clutter_text_buffer_set_text(self.as_buffer(), text.to_c_str().unwrap() as *mut i8, -1);
    }
  }

  /// Retrieves the contents of the buffer.
  ///
  /// The memory pointer returned by this call will not change unless this
  /// object emits a signal, or is finalized.
  ///
  /// _Since 1.10_
  fn get_text(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_text_buffer_get_text(self.as_buffer());
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  /// Retrieves the length in bytes of the buffer. See `.get_length()`.
  ///
  /// _Since 1.10_
  fn get_bytes(&mut self) -> u32 {
    unsafe {
      let foreign_result = clutter_text_buffer_get_bytes(self.as_buffer());
      return foreign_result;
    }
  }

  /// Retrieves the length in characters of the buffer.
  ///
  /// _Since 1.10_
  fn get_length(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_text_buffer_get_length(self.as_buffer());
      return foreign_result;
    }
  }

  /// Sets the maximum allowed length of the contents of the buffer.
  ///
  /// If the current contents are longer than the given length, then they will
  /// be truncated to fit.
  ///
  /// _Since 1.10_
  fn set_max_length(&mut self, max_length: i32) {
    unsafe {
      clutter_text_buffer_set_max_length(self.as_buffer(), max_length);
    }
  }

  /// Retrieves the maximum allowed length of the text in the buffer.
  ///
  /// _Since 1.10_
  fn get_max_length(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_text_buffer_get_max_length(self.as_buffer());
      return foreign_result;
    }
  }

  /// Inserts `chars` into the contents of the buffer, at position `position`.
  ///
  /// If `position` is out of bounds, or the maximum buffer text length is
  /// exceeded, then they are coerced to sane values.
  ///
  /// Note that the position is in characters, not in bytes.
  ///
  /// Returns the number of characters actually inserted.
  ///
  /// _Since 1.10_
  fn insert_text(&mut self, position: i32, chars: &str) -> i32 {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_buffer_insert_text(self.as_buffer(), position, chars.to_c_str().unwrap() as *mut i8, -1);
      return foreign_result;
    }
  }

  /// Deletes a sequence of characters from the buffer.
  ///
  /// `n_chars` characters are deleted starting at `position`. If `n_chars` is
  /// negative, then all characters until the end of the text are deleted.
  ///
  /// If `position` or `n_chars` are out of bounds, then they are coerced to
  /// sane values.
  ///
  /// Note that the positions are specified in characters, not bytes.
  ///
  /// Return the number of characters deleted.
  ///
  /// _Since 1.10_
  fn delete_text(&mut self, position: i32, n_chars: i32) -> i32 {
    unsafe {
      let foreign_result = clutter_text_buffer_delete_text(self.as_buffer(), position, n_chars);
      return foreign_result;
    }
  }

  /// Emits the `inserted-text` signal on the buffer.
  ///
  /// _Since 1.10_
  fn emit_inserted_text(&mut self, position: i32, chars: &str) -> i32 {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_text_buffer_emit_inserted_text(self.as_buffer(), position, chars.to_c_str().unwrap() as *mut i8, -1);
      return foreign_result;
    }
  }

  /// Emits the `deleted-text` signal on the buffer.
  ///
  /// _Since 1.10_
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
