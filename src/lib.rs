extern crate cairo;
extern crate libc;

#[link(name = "gobject-2.0")] extern {}
#[link(name = "clutter-1.0")] extern {}

pub fn init() {
  unsafe {
    clutter_init(std::ptr::mut_null(), std::ptr::mut_null());
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

pub mod scaling {
  #[repr(i32)]
  pub enum Filter {
    Linear = 0,
    Nearest = 1,
    Trilinear = 2
  }
}

pub mod color {
  use libc;
  use std;

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
}

pub mod actor {
  use libc;
  use std;
  use cairo;
  use super::content::Content;
  use super::constraint::Constraint;

  #[repr(i32)]
  pub enum Flags {
    Mapped = 2,
    Realized = 4,
    Reactive = 8,
    Visible = 16,
    NoLayout = 32
  }

  pub struct Box {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32
  }

  pub struct ActorRef {
    opaque: *mut libc::c_void
  }

  impl ActorRef {
    pub fn new() -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_new();
        return foreign_result;
      }
    }
  }

  pub trait Actor {
    fn as_actor(&self) -> *mut libc::c_void;

    fn set_flags(&mut self, flags: Flags) {
      unsafe {
        clutter_actor_set_flags(self.as_actor(), flags);
      }
    }

    fn unset_flags(&mut self, flags: Flags) {
      unsafe {
        clutter_actor_unset_flags(self.as_actor(), flags);
      }
    }

    fn get_flags(&mut self) -> Flags {
      unsafe {
        let foreign_result = clutter_actor_get_flags(self.as_actor());
        return foreign_result;
      }
    }

    fn set_name(&mut self, name: &str) {
      unsafe {
        use std::c_str::ToCStr;
        clutter_actor_set_name(self.as_actor(), name.to_c_str().unwrap() as *mut i8);
      }
    }

    fn get_name(&mut self) -> std::c_str::CString {
      unsafe {
        let foreign_result = clutter_actor_get_name(self.as_actor());
        return std::c_str::CString::new(foreign_result as *const i8, false);
      }
    }

    fn get_gid(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_gid(self.as_actor());
        return foreign_result;
      }
    }

    fn show(&mut self) {
      unsafe {
        clutter_actor_show(self.as_actor());
      }
    }

    fn show_all(&mut self) {
      unsafe {
        clutter_actor_show_all(self.as_actor());
      }
    }

    fn hide(&mut self) {
      unsafe {
        clutter_actor_hide(self.as_actor());
      }
    }

    fn hide_all(&mut self) {
      unsafe {
        clutter_actor_hide_all(self.as_actor());
      }
    }

    fn realize(&mut self) {
      unsafe {
        clutter_actor_realize(self.as_actor());
      }
    }

    fn unrealize(&mut self) {
      unsafe {
        clutter_actor_unrealize(self.as_actor());
      }
    }

    fn paint(&mut self) {
      unsafe {
        clutter_actor_paint(self.as_actor());
      }
    }

    fn continue_paint(&mut self) {
      unsafe {
        clutter_actor_continue_paint(self.as_actor());
      }
    }

    fn queue_redraw(&mut self) {
      unsafe {
        clutter_actor_queue_redraw(self.as_actor());
      }
    }

    fn queue_redraw_with_clip(&mut self, clip: &mut cairo::region::Rectangle) {
      unsafe {
        clutter_actor_queue_redraw_with_clip(self.as_actor(), clip);
      }
    }

    fn queue_relayout(&mut self) {
      unsafe {
        clutter_actor_queue_relayout(self.as_actor());
      }
    }

    fn should_pick_repaint(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_should_pick_repaint(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn map(&mut self) {
      unsafe {
        clutter_actor_map(self.as_actor());
      }
    }

    fn unmap(&mut self) {
      unsafe {
        clutter_actor_unmap(self.as_actor());
      }
    }

    fn has_overlaps(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_overlaps(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_mapped_clones(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_mapped_clones(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn get_preferred_size(&mut self) -> (f32, f32, f32, f32) {
      unsafe {
        let mut min_width_p:f32 = std::intrinsics::init();
        let mut min_height_p:f32 = std::intrinsics::init();
        let mut natural_width_p:f32 = std::intrinsics::init();
        let mut natural_height_p:f32 = std::intrinsics::init();
        clutter_actor_get_preferred_size(self.as_actor(), &mut min_width_p, &mut min_height_p, &mut natural_width_p, &mut natural_height_p);
        return (min_width_p, min_height_p, natural_width_p, natural_height_p);
      }
    }

    fn get_preferred_width(&mut self, for_height: f32) -> (f32, f32) {
      unsafe {
        let mut min_width_p:f32 = std::intrinsics::init();
        let mut natural_width_p:f32 = std::intrinsics::init();
        clutter_actor_get_preferred_width(self.as_actor(), for_height, &mut min_width_p, &mut natural_width_p);
        return (min_width_p, natural_width_p);
      }
    }

    fn get_preferred_height(&mut self, for_width: f32) -> (f32, f32) {
      unsafe {
        let mut min_height_p:f32 = std::intrinsics::init();
        let mut natural_height_p:f32 = std::intrinsics::init();
        clutter_actor_get_preferred_height(self.as_actor(), for_width, &mut min_height_p, &mut natural_height_p);
        return (min_height_p, natural_height_p);
      }
    }

    fn set_fixed_position_set(&mut self, fixed_position_set: bool) {
      unsafe {
        clutter_actor_set_fixed_position_set(self.as_actor(), (fixed_position_set as i32));
      }
    }

    fn get_fixed_position_set(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_fixed_position_set(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_allocation(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_allocation(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_x_expand(&mut self, expand: bool) {
      unsafe {
        clutter_actor_set_x_expand(self.as_actor(), (expand as i32));
      }
    }

    fn get_x_expand(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_x_expand(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_y_expand(&mut self, expand: bool) {
      unsafe {
        clutter_actor_set_y_expand(self.as_actor(), (expand as i32));
      }
    }

    fn get_y_expand(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_y_expand(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn needs_expand(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_needs_expand(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_size(&mut self, for_width: f32, for_height: f32) {
      unsafe {
        clutter_actor_set_size(self.as_actor(), for_width, for_height);
      }
    }

    fn get_size(&mut self) -> (f32, f32) {
      unsafe {
        let mut width:f32 = std::intrinsics::init();
        let mut height:f32 = std::intrinsics::init();
        clutter_actor_get_size(self.as_actor(), &mut width, &mut height);
        return (width, height);
      }
    }

    fn set_position(&mut self, x: f32, y: f32) {
      unsafe {
        clutter_actor_set_position(self.as_actor(), x, y);
      }
    }

    fn get_position(&mut self) -> (f32, f32) {
      unsafe {
        let mut x:f32 = std::intrinsics::init();
        let mut y:f32 = std::intrinsics::init();
        clutter_actor_get_position(self.as_actor(), &mut x, &mut y);
        return (x, y);
      }
    }

    fn set_width(&mut self, width: f32) {
      unsafe {
        clutter_actor_set_width(self.as_actor(), width);
      }
    }

    fn get_width(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_width(self.as_actor());
        return foreign_result;
      }
    }

    fn set_height(&mut self, height: f32) {
      unsafe {
        clutter_actor_set_height(self.as_actor(), height);
      }
    }

    fn get_height(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_height(self.as_actor());
        return foreign_result;
      }
    }

    fn set_x(&mut self, x: f32) {
      unsafe {
        clutter_actor_set_x(self.as_actor(), x);
      }
    }

    fn get_x(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_x(self.as_actor());
        return foreign_result;
      }
    }

    fn set_y(&mut self, y: f32) {
      unsafe {
        clutter_actor_set_y(self.as_actor(), y);
      }
    }

    fn get_y(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_y(self.as_actor());
        return foreign_result;
      }
    }

    fn move_by(&mut self, dx: f32, dy: f32) {
      unsafe {
        clutter_actor_move_by(self.as_actor(), dx, dy);
      }
    }

    fn set_z_position(&mut self, y: f32) {
      unsafe {
        clutter_actor_set_z_position(self.as_actor(), y);
      }
    }

    fn get_z_position(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_z_position(self.as_actor());
        return foreign_result;
      }
    }

    fn set_pivotal_point(&mut self, pivot_x: f32, pivot_y: f32) {
      unsafe {
        clutter_actor_set_pivotal_point(self.as_actor(), pivot_x, pivot_y);
      }
    }

    fn get_pivotal_point(&mut self) -> (f32, f32) {
      unsafe {
        let mut pivot_x:f32 = std::intrinsics::init();
        let mut pivot_y:f32 = std::intrinsics::init();
        clutter_actor_get_pivotal_point(self.as_actor(), &mut pivot_x, &mut pivot_y);
        return (pivot_x, pivot_y);
      }
    }

    fn set_pivot_point_z(&mut self, pivot_z: f32) {
      unsafe {
        clutter_actor_set_pivot_point_z(self.as_actor(), pivot_z);
      }
    }

    fn get_pivot_point_z(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_pivot_point_z(self.as_actor());
        return foreign_result;
      }
    }

    fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
      unsafe {
        clutter_actor_set_scale(self.as_actor(), scale_x, scale_y);
      }
    }

    fn get_scale(&mut self) -> (f32, f32) {
      unsafe {
        let mut scale_x:f32 = std::intrinsics::init();
        let mut scale_y:f32 = std::intrinsics::init();
        clutter_actor_get_scale(self.as_actor(), &mut scale_x, &mut scale_y);
        return (scale_x, scale_y);
      }
    }

    fn set_scale_z(&mut self, pivot_z: f32) {
      unsafe {
        clutter_actor_set_scale_z(self.as_actor(), pivot_z);
      }
    }

    fn get_scale_z(&mut self) -> f32 {
      unsafe {
        let foreign_result = clutter_actor_get_scale_z(self.as_actor());
        return foreign_result;
      }
    }

    fn is_scaled(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_is_scaled(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn is_rotated(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_is_rotated(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_translation(&mut self, translate_x: f32, translate_y: f32, translate_z: f32) {
      unsafe {
        clutter_actor_set_translation(self.as_actor(), translate_x, translate_y, translate_z);
      }
    }

    fn get_translation(&mut self) -> (f32, f32, f32) {
      unsafe {
        let mut translate_x:f32 = std::intrinsics::init();
        let mut translate_y:f32 = std::intrinsics::init();
        let mut translate_z:f32 = std::intrinsics::init();
        clutter_actor_get_translation(self.as_actor(), &mut translate_x, &mut translate_y, &mut translate_z);
        return (translate_x, translate_y, translate_z);
      }
    }

    fn apply_transform_to_point(&mut self, x: f32, y: f32) -> (bool, f32, f32) {
      unsafe {
        let mut x_out:f32 = std::intrinsics::init();
        let mut y_out:f32 = std::intrinsics::init();
        let foreign_result = clutter_actor_apply_transform_to_point(self.as_actor(), x, y, &mut x_out, &mut y_out);
        return (foreign_result != 0, x_out, y_out);
      }
    }

    fn transform_stage_point(&mut self, x: f32, y: f32) -> (bool, f32, f32) {
      unsafe {
        let mut x_out:f32 = std::intrinsics::init();
        let mut y_out:f32 = std::intrinsics::init();
        let foreign_result = clutter_actor_transform_stage_point(self.as_actor(), x, y, &mut x_out, &mut y_out);
        return (foreign_result != 0, x_out, y_out);
      }
    }

    fn get_transformed_position(&mut self) -> (f32, f32) {
      unsafe {
        let mut x:f32 = std::intrinsics::init();
        let mut y:f32 = std::intrinsics::init();
        clutter_actor_get_transformed_position(self.as_actor(), &mut x, &mut y);
        return (x, y);
      }
    }

    fn get_transformed_size(&mut self) -> (f32, f32) {
      unsafe {
        let mut width:f32 = std::intrinsics::init();
        let mut height:f32 = std::intrinsics::init();
        clutter_actor_get_transformed_size(self.as_actor(), &mut width, &mut height);
        return (width, height);
      }
    }

    fn get_paint_opacity(&mut self) -> i8 {
      unsafe {
        let foreign_result = clutter_actor_get_paint_opacity(self.as_actor());
        return foreign_result;
      }
    }

    fn get_paint_visibility(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_paint_visibility(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_content<T: Content>(&mut self, content: &mut T) {
      unsafe {
        clutter_actor_set_content(self.as_actor(), content.as_content());
      }
    }

    fn get_content(&mut self) -> super::content::ContentRef {
      unsafe {
        let foreign_result = clutter_actor_get_content(self.as_actor());
        return foreign_result;
      }
    }

    fn set_content_scaling_filters(&mut self, min: super::scaling::Filter, mag: super::scaling::Filter) {
      unsafe {
        clutter_actor_set_content_scaling_filters(self.as_actor(), min, mag);
      }
    }

    fn get_content_scaling_filters(&mut self) -> (super::scaling::Filter, super::scaling::Filter) {
      unsafe {
        let mut min:super::scaling::Filter = std::intrinsics::init();
        let mut mag:super::scaling::Filter = std::intrinsics::init();
        clutter_actor_get_content_scaling_filters(self.as_actor(), &mut min, &mut mag);
        return (min, mag);
      }
    }

    fn set_clip(&mut self, xoff: f32, yoff: f32, width: f32, height: f32) {
      unsafe {
        clutter_actor_set_clip(self.as_actor(), xoff, yoff, width, height);
      }
    }

    fn remove_clip(&mut self) {
      unsafe {
        clutter_actor_remove_clip(self.as_actor());
      }
    }

    fn has_clip(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_clip(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn get_clip(&mut self) -> (f32, f32, f32, f32) {
      unsafe {
        let mut xoff:f32 = std::intrinsics::init();
        let mut yoff:f32 = std::intrinsics::init();
        let mut width:f32 = std::intrinsics::init();
        let mut height:f32 = std::intrinsics::init();
        clutter_actor_get_clip(self.as_actor(), &mut xoff, &mut yoff, &mut width, &mut height);
        return (xoff, yoff, width, height);
      }
    }

    fn set_clip_to_allocation(&mut self, clip_set: bool) {
      unsafe {
        clutter_actor_set_clip_to_allocation(self.as_actor(), (clip_set as i32));
      }
    }

    fn get_clip_to_allocation(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_clip_to_allocation(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn set_opacity(&mut self, opacity: i8) {
      unsafe {
        clutter_actor_set_opacity(self.as_actor(), opacity);
      }
    }

    fn get_opacity(&mut self) -> i8 {
      unsafe {
        let foreign_result = clutter_actor_get_opacity(self.as_actor());
        return foreign_result;
      }
    }

    fn is_in_clone_paint(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_is_in_clone_paint(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn add_child<T: Actor>(&mut self, child: &mut T) {
      unsafe {
        clutter_actor_add_child(self.as_actor(), child.as_actor());
      }
    }

    fn insert_child_above<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
      unsafe {
        clutter_actor_insert_child_above(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn insert_child_at_index<T: Actor>(&mut self, child: &mut T, index: i32) {
      unsafe {
        clutter_actor_insert_child_at_index(self.as_actor(), child.as_actor(), index);
      }
    }

    fn insert_child_below<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
      unsafe {
        clutter_actor_insert_child_below(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn replace_child<T: Actor, U: Actor>(&mut self, old_child: &mut T, new_child: &mut U) {
      unsafe {
        clutter_actor_replace_child(self.as_actor(), old_child.as_actor(), new_child.as_actor());
      }
    }

    fn remove_child<T: Actor>(&mut self, child: &mut T) {
      unsafe {
        clutter_actor_remove_child(self.as_actor(), child.as_actor());
      }
    }

    fn remove_all_children(&mut self) {
      unsafe {
        clutter_actor_remove_all_children(self.as_actor());
      }
    }

    fn destroy_all_children(&mut self) {
      unsafe {
        clutter_actor_destroy_all_children(self.as_actor());
      }
    }

    fn get_first_child(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_first_child(self.as_actor());
        return foreign_result;
      }
    }

    fn get_next_sibling(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_next_sibling(self.as_actor());
        return foreign_result;
      }
    }

    fn get_previous_sibling(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_previous_sibling(self.as_actor());
        return foreign_result;
      }
    }

    fn get_last_child(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_last_child(self.as_actor());
        return foreign_result;
      }
    }

    fn get_child_at_index(&mut self, index: i32) {
      unsafe {
        clutter_actor_get_child_at_index(self.as_actor(), index);
      }
    }

    fn get_n_children(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_n_children(self.as_actor());
        return foreign_result;
      }
    }

    fn get_parent(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_parent(self.as_actor());
        return foreign_result;
      }
    }

    fn set_child_above_sibling<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
      unsafe {
        clutter_actor_set_child_above_sibling(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn set_child_at_index<T: Actor>(&mut self, child: &mut T, index: i32) {
      unsafe {
        clutter_actor_set_child_at_index(self.as_actor(), child.as_actor(), index);
      }
    }

    fn set_child_below_sibling<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
      unsafe {
        clutter_actor_set_child_below_sibling(self.as_actor(), child.as_actor(), sibling.as_actor());
      }
    }

    fn contains<T: Actor>(&mut self, descendant: &mut T) -> bool {
      unsafe {
        let foreign_result = clutter_actor_contains(self.as_actor(), descendant.as_actor());
        return foreign_result != 0;
      }
    }

    fn get_stage(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_get_stage(self.as_actor());
        return foreign_result;
      }
    }

    fn save_easing_state(&mut self) {
      unsafe {
        clutter_actor_save_easing_state(self.as_actor());
      }
    }

    fn restore_easing_state(&mut self) {
      unsafe {
        clutter_actor_restore_easing_state(self.as_actor());
      }
    }

    fn set_easing_duration(&mut self, msecs: i32) {
      unsafe {
        clutter_actor_set_easing_duration(self.as_actor(), msecs);
      }
    }

    fn get_easing_duration(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_easing_duration(self.as_actor());
        return foreign_result;
      }
    }

    fn set_easing_delay(&mut self, msecs: i32) {
      unsafe {
        clutter_actor_set_easing_delay(self.as_actor(), msecs);
      }
    }

    fn get_easing_delay(&mut self) -> i32 {
      unsafe {
        let foreign_result = clutter_actor_get_easing_delay(self.as_actor());
        return foreign_result;
      }
    }

    fn set_reactive(&mut self, reactive: bool) {
      unsafe {
        clutter_actor_set_reactive(self.as_actor(), (reactive as i32));
      }
    }

    fn get_reactive(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_get_reactive(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_key_focus(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_key_focus(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn grap_key_focus(&mut self) {
      unsafe {
        clutter_actor_grap_key_focus(self.as_actor());
      }
    }

    fn has_pointer(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_pointer(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn has_actions(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_has_actions(self.as_actor());
        return foreign_result != 0;
      }
    }

    fn add_constraint<T: Constraint>(&mut self, constraint: &mut T) {
      unsafe {
        clutter_actor_add_constraint(self.as_actor(), constraint.as_constraint());
      }
    }

    fn on_allocation_changed(&mut self, handler: &|&mut ActorRef, &Box, allocation::Flags|) -> u64 {
      unsafe {
        let null_void: *mut libc::c_void = std::ptr::null_mut();
        return rsi_connect_on_allocation_changed(self.as_actor(), "allocation_changed".to_c_str().unwrap() as *mut i8, handler_for_on_allocation_changed, std::mem::transmute::<&|&mut ActorRef, &Box, allocation::Flags|, *mut libc::c_void>(handler), null_void, 0);
      }
    }

    fn on_destroy(&mut self, handler: &|&mut ActorRef|) -> u64 {
      unsafe {
        let null_void: *mut libc::c_void = std::ptr::null_mut();
        return rsi_connect_on_destroy(self.as_actor(), "destroy".to_c_str().unwrap() as *mut i8, handler_for_on_destroy, std::mem::transmute::<&|&mut ActorRef|, *mut libc::c_void>(handler), null_void, 0);
      }
    }
  }

  impl Actor for ActorRef {
    fn as_actor(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  extern "C" fn handler_for_on_allocation_changed(actor: *mut libc::c_void, allocation_box: *mut Box, flags: allocation::Flags, handler: *mut libc::c_void) {
    unsafe {
      let mut actor_r = ActorRef { opaque: actor };
      let handler = std::mem::transmute::<*mut libc::c_void, &mut |actor: &mut ActorRef, allocation_box: &Box, flags: allocation::Flags|>(handler);
      (*handler)(&mut actor_r, std::mem::transmute(allocation_box), flags);
      std::mem::forget(actor_r);
    }
  }

  extern "C" fn handler_for_on_destroy(actor: *mut libc::c_void, handler: *mut libc::c_void) {
    unsafe {
      let mut actor_r = ActorRef { opaque: actor };
      let handler = std::mem::transmute::<*mut libc::c_void, &mut |actor: &mut ActorRef|>(handler);
      (*handler)(&mut actor_r);
      std::mem::forget(actor_r);
    }
  }

  extern {
    fn clutter_actor_new() -> ActorRef;
    #[link_name = "g_signal_connect_data"]
    fn rsi_connect_on_allocation_changed(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut Box, allocation::Flags, *mut libc::c_void), data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
    #[link_name = "g_signal_connect_data"]
    fn rsi_connect_on_destroy(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut libc::c_void), data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
    fn clutter_actor_set_flags(self_value: *mut libc::c_void, flags: Flags);
    fn clutter_actor_unset_flags(self_value: *mut libc::c_void, flags: Flags);
    fn clutter_actor_get_flags(self_value: *mut libc::c_void) -> Flags;
    fn clutter_actor_set_name(self_value: *mut libc::c_void, name: *mut libc::c_char);
    fn clutter_actor_get_name(self_value: *mut libc::c_void) -> *mut i8;
    fn clutter_actor_get_gid(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_show(self_value: *mut libc::c_void);
    fn clutter_actor_show_all(self_value: *mut libc::c_void);
    fn clutter_actor_hide(self_value: *mut libc::c_void);
    fn clutter_actor_hide_all(self_value: *mut libc::c_void);
    fn clutter_actor_realize(self_value: *mut libc::c_void);
    fn clutter_actor_unrealize(self_value: *mut libc::c_void);
    fn clutter_actor_paint(self_value: *mut libc::c_void);
    fn clutter_actor_continue_paint(self_value: *mut libc::c_void);
    fn clutter_actor_queue_redraw(self_value: *mut libc::c_void);
    fn clutter_actor_queue_redraw_with_clip(self_value: *mut libc::c_void, clip: *mut cairo::region::Rectangle);
    fn clutter_actor_queue_relayout(self_value: *mut libc::c_void);
    fn clutter_actor_should_pick_repaint(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_map(self_value: *mut libc::c_void);
    fn clutter_actor_unmap(self_value: *mut libc::c_void);
    fn clutter_actor_has_overlaps(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_has_mapped_clones(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_get_preferred_size(self_value: *mut libc::c_void, min_width_p: *mut f32, min_height_p: *mut f32, natural_width_p: *mut f32, natural_height_p: *mut f32);
    fn clutter_actor_get_preferred_width(self_value: *mut libc::c_void, for_height: f32, min_width_p: *mut f32, natural_width_p: *mut f32);
    fn clutter_actor_get_preferred_height(self_value: *mut libc::c_void, for_width: f32, min_height_p: *mut f32, natural_height_p: *mut f32);
    fn clutter_actor_set_fixed_position_set(self_value: *mut libc::c_void, fixed_position_set: i32);
    fn clutter_actor_get_fixed_position_set(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_has_allocation(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_x_expand(self_value: *mut libc::c_void, expand: i32);
    fn clutter_actor_get_x_expand(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_y_expand(self_value: *mut libc::c_void, expand: i32);
    fn clutter_actor_get_y_expand(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_needs_expand(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_size(self_value: *mut libc::c_void, for_width: f32, for_height: f32);
    fn clutter_actor_get_size(self_value: *mut libc::c_void, width: *mut f32, height: *mut f32);
    fn clutter_actor_set_position(self_value: *mut libc::c_void, x: f32, y: f32);
    fn clutter_actor_get_position(self_value: *mut libc::c_void, x: *mut f32, y: *mut f32);
    fn clutter_actor_set_width(self_value: *mut libc::c_void, width: f32);
    fn clutter_actor_get_width(self_value: *mut libc::c_void) -> f32;
    fn clutter_actor_set_height(self_value: *mut libc::c_void, height: f32);
    fn clutter_actor_get_height(self_value: *mut libc::c_void) -> f32;
    fn clutter_actor_set_x(self_value: *mut libc::c_void, x: f32);
    fn clutter_actor_get_x(self_value: *mut libc::c_void) -> f32;
    fn clutter_actor_set_y(self_value: *mut libc::c_void, y: f32);
    fn clutter_actor_get_y(self_value: *mut libc::c_void) -> f32;
    fn clutter_actor_move_by(self_value: *mut libc::c_void, dx: f32, dy: f32);
    fn clutter_actor_set_z_position(self_value: *mut libc::c_void, y: f32);
    fn clutter_actor_get_z_position(self_value: *mut libc::c_void) -> f32;
    fn clutter_actor_set_pivotal_point(self_value: *mut libc::c_void, pivot_x: f32, pivot_y: f32);
    fn clutter_actor_get_pivotal_point(self_value: *mut libc::c_void, pivot_x: *mut f32, pivot_y: *mut f32);
    fn clutter_actor_set_pivot_point_z(self_value: *mut libc::c_void, pivot_z: f32);
    fn clutter_actor_get_pivot_point_z(self_value: *mut libc::c_void) -> f32;
    fn clutter_actor_set_scale(self_value: *mut libc::c_void, scale_x: f32, scale_y: f32);
    fn clutter_actor_get_scale(self_value: *mut libc::c_void, scale_x: *mut f32, scale_y: *mut f32);
    fn clutter_actor_set_scale_z(self_value: *mut libc::c_void, pivot_z: f32);
    fn clutter_actor_get_scale_z(self_value: *mut libc::c_void) -> f32;
    fn clutter_actor_is_scaled(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_is_rotated(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_translation(self_value: *mut libc::c_void, translate_x: f32, translate_y: f32, translate_z: f32);
    fn clutter_actor_get_translation(self_value: *mut libc::c_void, translate_x: *mut f32, translate_y: *mut f32, translate_z: *mut f32);
    fn clutter_actor_apply_transform_to_point(self_value: *mut libc::c_void, x: f32, y: f32, x_out: *mut f32, y_out: *mut f32) -> i32;
    fn clutter_actor_transform_stage_point(self_value: *mut libc::c_void, x: f32, y: f32, x_out: *mut f32, y_out: *mut f32) -> i32;
    fn clutter_actor_get_transformed_position(self_value: *mut libc::c_void, x: *mut f32, y: *mut f32);
    fn clutter_actor_get_transformed_size(self_value: *mut libc::c_void, width: *mut f32, height: *mut f32);
    fn clutter_actor_get_paint_opacity(self_value: *mut libc::c_void) -> i8;
    fn clutter_actor_get_paint_visibility(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_content(self_value: *mut libc::c_void, content: *mut libc::c_void);
    fn clutter_actor_get_content(self_value: *mut libc::c_void) -> super::content::ContentRef;
    fn clutter_actor_set_content_scaling_filters(self_value: *mut libc::c_void, min: super::scaling::Filter, mag: super::scaling::Filter);
    fn clutter_actor_get_content_scaling_filters(self_value: *mut libc::c_void, min: *mut super::scaling::Filter, mag: *mut super::scaling::Filter);
    fn clutter_actor_set_clip(self_value: *mut libc::c_void, xoff: f32, yoff: f32, width: f32, height: f32);
    fn clutter_actor_remove_clip(self_value: *mut libc::c_void);
    fn clutter_actor_has_clip(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_get_clip(self_value: *mut libc::c_void, xoff: *mut f32, yoff: *mut f32, width: *mut f32, height: *mut f32);
    fn clutter_actor_set_clip_to_allocation(self_value: *mut libc::c_void, clip_set: i32);
    fn clutter_actor_get_clip_to_allocation(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_opacity(self_value: *mut libc::c_void, opacity: i8);
    fn clutter_actor_get_opacity(self_value: *mut libc::c_void) -> i8;
    fn clutter_actor_is_in_clone_paint(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_add_child(self_value: *mut libc::c_void, child: *mut libc::c_void);
    fn clutter_actor_insert_child_above(self_value: *mut libc::c_void, child: *mut libc::c_void, sibling: *mut libc::c_void);
    fn clutter_actor_insert_child_at_index(self_value: *mut libc::c_void, child: *mut libc::c_void, index: i32);
    fn clutter_actor_insert_child_below(self_value: *mut libc::c_void, child: *mut libc::c_void, sibling: *mut libc::c_void);
    fn clutter_actor_replace_child(self_value: *mut libc::c_void, old_child: *mut libc::c_void, new_child: *mut libc::c_void);
    fn clutter_actor_remove_child(self_value: *mut libc::c_void, child: *mut libc::c_void);
    fn clutter_actor_remove_all_children(self_value: *mut libc::c_void);
    fn clutter_actor_destroy_all_children(self_value: *mut libc::c_void);
    fn clutter_actor_get_first_child(self_value: *mut libc::c_void) -> ActorRef;
    fn clutter_actor_get_next_sibling(self_value: *mut libc::c_void) -> ActorRef;
    fn clutter_actor_get_previous_sibling(self_value: *mut libc::c_void) -> ActorRef;
    fn clutter_actor_get_last_child(self_value: *mut libc::c_void) -> ActorRef;
    fn clutter_actor_get_child_at_index(self_value: *mut libc::c_void, index: i32);
    fn clutter_actor_get_n_children(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_get_parent(self_value: *mut libc::c_void) -> ActorRef;
    fn clutter_actor_set_child_above_sibling(self_value: *mut libc::c_void, child: *mut libc::c_void, sibling: *mut libc::c_void);
    fn clutter_actor_set_child_at_index(self_value: *mut libc::c_void, child: *mut libc::c_void, index: i32);
    fn clutter_actor_set_child_below_sibling(self_value: *mut libc::c_void, child: *mut libc::c_void, sibling: *mut libc::c_void);
    fn clutter_actor_contains(self_value: *mut libc::c_void, descendant: *mut libc::c_void) -> i32;
    fn clutter_actor_get_stage(self_value: *mut libc::c_void) -> ActorRef;
    fn clutter_actor_save_easing_state(self_value: *mut libc::c_void);
    fn clutter_actor_restore_easing_state(self_value: *mut libc::c_void);
    fn clutter_actor_set_easing_duration(self_value: *mut libc::c_void, msecs: i32);
    fn clutter_actor_get_easing_duration(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_easing_delay(self_value: *mut libc::c_void, msecs: i32);
    fn clutter_actor_get_easing_delay(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_set_reactive(self_value: *mut libc::c_void, reactive: i32);
    fn clutter_actor_get_reactive(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_has_key_focus(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_grap_key_focus(self_value: *mut libc::c_void);
    fn clutter_actor_has_pointer(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_has_actions(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_add_constraint(self_value: *mut libc::c_void, constraint: *mut libc::c_void);
  }

  pub struct ActorMetaRef {
    opaque: *mut libc::c_void
  }

  pub trait ActorMeta {
    fn as_actor_meta(&self) -> *mut libc::c_void;

    fn set_name(&mut self, name: &str) {
      unsafe {
        use std::c_str::ToCStr;
        clutter_actor_meta_set_name(self.as_actor_meta(), name.to_c_str().unwrap() as *mut i8);
      }
    }

    fn get_name(&mut self) -> std::c_str::CString {
      unsafe {
        let foreign_result = clutter_actor_meta_get_name(self.as_actor_meta());
        return std::c_str::CString::new(foreign_result as *const i8, false);
      }
    }

    fn set_enabled(&mut self, enabled: bool) {
      unsafe {
        clutter_actor_meta_set_enabled(self.as_actor_meta(), (enabled as i32));
      }
    }

    fn get_enabled(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_actor_meta_get_enabled(self.as_actor_meta());
        return foreign_result != 0;
      }
    }

    fn get_actor(&mut self) -> ActorRef {
      unsafe {
        let foreign_result = clutter_actor_meta_get_actor(self.as_actor_meta());
        return foreign_result;
      }
    }
  }

  impl ActorMeta for ActorMetaRef {
    fn as_actor_meta(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  extern {
    fn clutter_actor_meta_set_name(self_value: *mut libc::c_void, name: *mut libc::c_char);
    fn clutter_actor_meta_get_name(self_value: *mut libc::c_void) -> *mut i8;
    fn clutter_actor_meta_set_enabled(self_value: *mut libc::c_void, enabled: i32);
    fn clutter_actor_meta_get_enabled(self_value: *mut libc::c_void) -> i32;
    fn clutter_actor_meta_get_actor(self_value: *mut libc::c_void) -> ActorRef;
  }

  impl std::ops::Drop for ActorRef {
    fn drop(&mut self) {
      unsafe {
        clutter_actor_destroy(self.opaque);
      }
    }
  }

  extern {
    fn clutter_actor_destroy(self_value: *mut libc::c_void);
  }

  pub mod allocation {
    #[repr(i32)]
    pub enum Flags {
      None = 0,
      AbsoluteOriginChanged = 2,
      DelegateLayout = 4
    }
  }
}

pub mod stage {
  use libc;
  use std;
  use super::actor::Actor;

  pub struct StageRef {
    opaque: *mut libc::c_void
  }

  impl StageRef {
    pub fn new() -> StageRef {
      unsafe {
        let foreign_result = clutter_stage_new();
        return foreign_result;
      }
    }
  }

  pub trait Stage {
    fn as_stage(&self) -> *mut libc::c_void;

    fn ensure_current(&mut self) {
      unsafe {
        clutter_stage_ensure_current(self.as_stage());
      }
    }

    fn ensure_viewport(&mut self) {
      unsafe {
        clutter_stage_ensure_viewport(self.as_stage());
      }
    }

    fn ensure_redraw(&mut self) {
      unsafe {
        clutter_stage_ensure_redraw(self.as_stage());
      }
    }

    fn set_key_focus<T: Actor>(&mut self, actor: &mut T) {
      unsafe {
        clutter_stage_set_key_focus(self.as_stage(), actor.as_actor());
      }
    }

    fn get_key_focus(&mut self) -> super::actor::ActorRef {
      unsafe {
        let foreign_result = clutter_stage_get_key_focus(self.as_stage());
        return foreign_result;
      }
    }

    fn set_throttle_motion_events(&mut self, throttle: bool) {
      unsafe {
        clutter_stage_set_throttle_motion_events(self.as_stage(), (throttle as i32));
      }
    }

    fn get_throttle_motion_events(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_throttle_motion_events(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_use_alpha(&mut self, use_alpha: bool) {
      unsafe {
        clutter_stage_set_use_alpha(self.as_stage(), (use_alpha as i32));
      }
    }

    fn get_use_alpha(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_use_alpha(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_minimum_size(&mut self, width: i32, height: i32) {
      unsafe {
        clutter_stage_set_minimum_size(self.as_stage(), width, height);
      }
    }

    fn get_minimum_size(&mut self) -> (i32, i32) {
      unsafe {
        let mut width:i32 = std::intrinsics::init();
        let mut height:i32 = std::intrinsics::init();
        clutter_stage_get_minimum_size(self.as_stage(), &mut width, &mut height);
        return (width, height);
      }
    }

    fn set_no_clear_hint(&mut self, use_alpha: bool) {
      unsafe {
        clutter_stage_set_no_clear_hint(self.as_stage(), (use_alpha as i32));
      }
    }

    fn get_no_clear_hint(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_no_clear_hint(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_motion_events_enabled(&mut self, enabled: bool) {
      unsafe {
        clutter_stage_set_motion_events_enabled(self.as_stage(), (enabled as i32));
      }
    }

    fn get_motion_events_enabled(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_motion_events_enabled(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_title(&mut self, title: &str) {
      unsafe {
        use std::c_str::ToCStr;
        clutter_stage_set_title(self.as_stage(), title.to_c_str().unwrap() as *mut i8);
      }
    }

    fn get_title(&mut self) -> std::c_str::CString {
      unsafe {
        let foreign_result = clutter_stage_get_title(self.as_stage());
        return std::c_str::CString::new(foreign_result as *const i8, false);
      }
    }

    fn set_user_resizable(&mut self, resizable: bool) {
      unsafe {
        clutter_stage_set_user_resizable(self.as_stage(), (resizable as i32));
      }
    }

    fn get_user_resizable(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_user_resizable(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
      unsafe {
        clutter_stage_set_fullscreen(self.as_stage(), (fullscreen as i32));
      }
    }

    fn get_fullscreen(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_fullscreen(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn show_cursor(&mut self) {
      unsafe {
        clutter_stage_show_cursor(self.as_stage());
      }
    }

    fn hide_cursor(&mut self) {
      unsafe {
        clutter_stage_hide_cursor(self.as_stage());
      }
    }

    fn set_accept_focus(&mut self, accept_focus: bool) {
      unsafe {
        clutter_stage_set_accept_focus(self.as_stage(), (accept_focus as i32));
      }
    }

    fn get_accept_focus(&mut self) -> bool {
      unsafe {
        let foreign_result = clutter_stage_get_accept_focus(self.as_stage());
        return foreign_result != 0;
      }
    }

    fn set_sync_delay(&mut self, accept_focus: i32) {
      unsafe {
        clutter_stage_set_sync_delay(self.as_stage(), accept_focus);
      }
    }

    fn skip_sync_delay(&mut self) {
      unsafe {
        clutter_stage_skip_sync_delay(self.as_stage());
      }
    }

    fn on_activate(&mut self, handler: &|&mut StageRef|) -> u64 {
      unsafe {
        let null_void: *mut libc::c_void = std::ptr::null_mut();
        return rsi_connect_on_activate(self.as_stage(), "activate".to_c_str().unwrap() as *mut i8, handler_for_on_activate, std::mem::transmute::<&|&mut StageRef|, *mut libc::c_void>(handler), null_void, 0);
      }
    }

    fn on_deactivate(&mut self, handler: &|&mut StageRef|) -> u64 {
      unsafe {
        let null_void: *mut libc::c_void = std::ptr::null_mut();
        return rsi_connect_on_deactivate(self.as_stage(), "deactivate".to_c_str().unwrap() as *mut i8, handler_for_on_deactivate, std::mem::transmute::<&|&mut StageRef|, *mut libc::c_void>(handler), null_void, 0);
      }
    }

    fn on_fullscreen(&mut self, handler: &|&mut StageRef|) -> u64 {
      unsafe {
        let null_void: *mut libc::c_void = std::ptr::null_mut();
        return rsi_connect_on_fullscreen(self.as_stage(), "fullscreen".to_c_str().unwrap() as *mut i8, handler_for_on_fullscreen, std::mem::transmute::<&|&mut StageRef|, *mut libc::c_void>(handler), null_void, 0);
      }
    }

    fn on_unfullscreen(&mut self, handler: &|&mut StageRef|) -> u64 {
      unsafe {
        let null_void: *mut libc::c_void = std::ptr::null_mut();
        return rsi_connect_on_unfullscreen(self.as_stage(), "unfullscreen".to_c_str().unwrap() as *mut i8, handler_for_on_unfullscreen, std::mem::transmute::<&|&mut StageRef|, *mut libc::c_void>(handler), null_void, 0);
      }
    }
  }

  impl Stage for StageRef {
    fn as_stage(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  impl Actor for StageRef {
    fn as_actor(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  extern "C" fn handler_for_on_activate(stage: *mut libc::c_void, handler: *mut libc::c_void) {
    unsafe {
      let mut stage_r = StageRef { opaque: stage };
      let handler = std::mem::transmute::<*mut libc::c_void, &mut |stage: &mut StageRef|>(handler);
      (*handler)(&mut stage_r);
      std::mem::forget(stage_r);
    }
  }

  extern "C" fn handler_for_on_deactivate(stage: *mut libc::c_void, handler: *mut libc::c_void) {
    unsafe {
      let mut stage_r = StageRef { opaque: stage };
      let handler = std::mem::transmute::<*mut libc::c_void, &mut |stage: &mut StageRef|>(handler);
      (*handler)(&mut stage_r);
      std::mem::forget(stage_r);
    }
  }

  extern "C" fn handler_for_on_fullscreen(stage: *mut libc::c_void, handler: *mut libc::c_void) {
    unsafe {
      let mut stage_r = StageRef { opaque: stage };
      let handler = std::mem::transmute::<*mut libc::c_void, &mut |stage: &mut StageRef|>(handler);
      (*handler)(&mut stage_r);
      std::mem::forget(stage_r);
    }
  }

  extern "C" fn handler_for_on_unfullscreen(stage: *mut libc::c_void, handler: *mut libc::c_void) {
    unsafe {
      let mut stage_r = StageRef { opaque: stage };
      let handler = std::mem::transmute::<*mut libc::c_void, &mut |stage: &mut StageRef|>(handler);
      (*handler)(&mut stage_r);
      std::mem::forget(stage_r);
    }
  }

  extern {
    fn clutter_stage_new() -> StageRef;
    #[link_name = "g_signal_connect_data"]
    fn rsi_connect_on_activate(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut libc::c_void), data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
    #[link_name = "g_signal_connect_data"]
    fn rsi_connect_on_deactivate(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut libc::c_void), data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
    #[link_name = "g_signal_connect_data"]
    fn rsi_connect_on_fullscreen(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut libc::c_void), data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
    #[link_name = "g_signal_connect_data"]
    fn rsi_connect_on_unfullscreen(instance: *mut libc::c_void, detailed_signal: *mut libc::c_char, c_handler: extern "C" fn(*mut libc::c_void, *mut libc::c_void), data: *mut libc::c_void, destroy_data: *mut libc::c_void, connect_flags: i32) -> u64;
    fn clutter_stage_ensure_current(self_value: *mut libc::c_void);
    fn clutter_stage_ensure_viewport(self_value: *mut libc::c_void);
    fn clutter_stage_ensure_redraw(self_value: *mut libc::c_void);
    fn clutter_stage_set_key_focus(self_value: *mut libc::c_void, actor: *mut libc::c_void);
    fn clutter_stage_get_key_focus(self_value: *mut libc::c_void) -> super::actor::ActorRef;
    fn clutter_stage_set_throttle_motion_events(self_value: *mut libc::c_void, throttle: i32);
    fn clutter_stage_get_throttle_motion_events(self_value: *mut libc::c_void) -> i32;
    fn clutter_stage_set_use_alpha(self_value: *mut libc::c_void, use_alpha: i32);
    fn clutter_stage_get_use_alpha(self_value: *mut libc::c_void) -> i32;
    fn clutter_stage_set_minimum_size(self_value: *mut libc::c_void, width: i32, height: i32);
    fn clutter_stage_get_minimum_size(self_value: *mut libc::c_void, width: *mut i32, height: *mut i32);
    fn clutter_stage_set_no_clear_hint(self_value: *mut libc::c_void, use_alpha: i32);
    fn clutter_stage_get_no_clear_hint(self_value: *mut libc::c_void) -> i32;
    fn clutter_stage_set_motion_events_enabled(self_value: *mut libc::c_void, enabled: i32);
    fn clutter_stage_get_motion_events_enabled(self_value: *mut libc::c_void) -> i32;
    fn clutter_stage_set_title(self_value: *mut libc::c_void, title: *mut libc::c_char);
    fn clutter_stage_get_title(self_value: *mut libc::c_void) -> *mut i8;
    fn clutter_stage_set_user_resizable(self_value: *mut libc::c_void, resizable: i32);
    fn clutter_stage_get_user_resizable(self_value: *mut libc::c_void) -> i32;
    fn clutter_stage_set_fullscreen(self_value: *mut libc::c_void, fullscreen: i32);
    fn clutter_stage_get_fullscreen(self_value: *mut libc::c_void) -> i32;
    fn clutter_stage_show_cursor(self_value: *mut libc::c_void);
    fn clutter_stage_hide_cursor(self_value: *mut libc::c_void);
    fn clutter_stage_set_accept_focus(self_value: *mut libc::c_void, accept_focus: i32);
    fn clutter_stage_get_accept_focus(self_value: *mut libc::c_void) -> i32;
    fn clutter_stage_set_sync_delay(self_value: *mut libc::c_void, accept_focus: i32);
    fn clutter_stage_skip_sync_delay(self_value: *mut libc::c_void);
  }
}

pub mod constraint {
  use libc;
  use std;
  use super::actor::Actor;
  use super::actor::ActorMeta;

  pub struct ConstraintRef {
    opaque: *mut libc::c_void
  }

  pub trait Constraint {
    fn as_constraint(&self) -> *mut libc::c_void;

  }

  impl Constraint for ConstraintRef {
    fn as_constraint(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  impl ActorMeta for ConstraintRef {
    fn as_actor_meta(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  extern {
  }

  pub struct BindConstraintRef {
    opaque: *mut libc::c_void
  }

  impl BindConstraintRef {
    pub fn new<T: Actor>(source: &mut T, coordinate: bind::Coordinate, offset: f32) -> BindConstraintRef {
      unsafe {
        let foreign_result = clutter_bind_constraint_new(source.as_actor(), coordinate, offset);
        return foreign_result;
      }
    }
  }

  pub trait BindConstraint {
    fn as_bind_constraint(&self) -> *mut libc::c_void;

  }

  impl BindConstraint for BindConstraintRef {
    fn as_bind_constraint(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  impl Constraint for BindConstraintRef {
    fn as_constraint(&self) -> *mut libc::c_void {
      return self.opaque;
    }
  }

  extern {
    fn clutter_bind_constraint_new(source: *mut libc::c_void, coordinate: bind::Coordinate, offset: f32) -> BindConstraintRef;
  }

  pub mod bind {
    #[repr(i32)]
    pub enum Coordinate {
      X = 0,
      Y = 1,
      Width = 2,
      Height = 3,
      Position = 4,
      Size = 5,
      All = 6
    }
  }
}

pub mod content {
  use libc;
  use std;

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
}

pub mod text {
  use libc;
  use std;
  use super::actor::Actor;
  use super::content::Content;

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

    fn get_bytes(&mut self) -> uint {
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
    fn clutter_text_buffer_get_bytes(self_value: *mut libc::c_void) -> uint;
    fn clutter_text_buffer_get_length(self_value: *mut libc::c_void) -> i32;
    fn clutter_text_buffer_set_max_length(self_value: *mut libc::c_void, max_length: i32);
    fn clutter_text_buffer_get_max_length(self_value: *mut libc::c_void) -> i32;
    fn clutter_text_buffer_insert_text(self_value: *mut libc::c_void, position: i32, chars: *mut libc::c_char, n_chars: i32) -> i32;
    fn clutter_text_buffer_delete_text(self_value: *mut libc::c_void, position: i32, n_chars: i32) -> i32;
    fn clutter_text_buffer_emit_inserted_text(self_value: *mut libc::c_void, position: i32, chars: *mut libc::c_char, n_chars: i32) -> i32;
    fn clutter_text_buffer_emit_deleted_text(self_value: *mut libc::c_void, position: i32, n_chars: i32) -> i32;
  }
}

pub mod canvas {
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
}

extern {
  fn clutter_init(argc: *mut i32, argv: *mut i32);
  fn clutter_main();
  fn clutter_main_quit();
}
