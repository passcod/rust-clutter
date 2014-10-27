#![stable]

use libc;
use std;

/// Named colors, for accessing global colors defined by Clutter.
#[repr(C)]
pub enum StaticColor {
  // CGA/EGA color palette
  White       =  0,
  Black       =  1,
  Red         =  2,
  DarkRed     =  3,
  Green       =  4,
  DarkGreen   =  5,
  Blue        =  6,
  DarkBlue    =  7,
  Cyan        =  8,
  DarkCyan    =  9,
  Magenta     = 10,
  DarkMagenta = 11,
  Yellow      = 12,
  DarkYellow  = 13,
  Gray        = 14,
  DarkGray    = 15,
  LightGray   = 16,

  // Tango Icon color palette
  Butter          = 17,
  ButterLight     = 18,
  ButterDark      = 19,
  Orange          = 20,
  OrangeLight     = 21,
  OrangeDark      = 22,
  Chocolate       = 23,
  ChocolateLight  = 24,
  ChocolateDark   = 25,
  Chameleon       = 26,
  ChameleonLight  = 27,
  ChameleonDark   = 28,
  SkyBlue         = 29,
  SkyBlueLight    = 30,
  SkyBlueDark     = 31,
  Plum            = 32,
  PlumLight       = 33,
  PlumDark        = 34,
  ScarletRed      = 35,
  ScarletRedLight = 36,
  ScarletRedDark  = 37,
  Aluminium1      = 38,
  Aluminium2      = 39,
  Aluminium3      = 40,
  Aluminium4      = 41,
  Aluminium5      = 42,
  Aluminium6      = 43,

  // Last color
  Transparent = 44
}

/// Color management and manipulation.
///
/// A Color is expressed as a 4-tuple of values ranging from zero to 255, one
/// for each RGB color channel plus one for the alpha.
///
/// The alpha channel is fully opaque at 255 and fully transparent at 0.
#[repr(C)]
pub struct Color {
  pub opaque: *mut libc::c_void
}

impl Color {
  /// Creates a new Color with the given values.
  ///
  /// _Since 0.8.4_
  pub fn new(red: i8, green: i8, blue: i8, alpha: i8) -> Color {
    unsafe {
      let foreign_result = clutter_color_new(red, green, blue, alpha);
      return foreign_result;
    }
  }

  /// Allocates a new, transparent black Color.
  ///
  /// _Since 1.12_
  pub fn alloc() -> Color {
    unsafe {
      let foreign_result = clutter_color_alloc();
      return foreign_result;
    }
  }

  /// Initializes color with the given values.
  ///
  /// _Since 1.12_
  pub fn init(&mut self, red: i8, green: i8, blue: i8, alpha: i8) {
    unsafe {
      clutter_color_init(self.opaque, red, green, blue, alpha);
    }
  }

  /// Compares two Colors and checks if they are the same.
  ///
  /// _Since 0.2_
  pub fn equal(&self, other: &Color) -> bool {
    unsafe {
      let foreign_result = clutter_color_equal(self.opaque as *mut libc::c_void, other.opaque as *mut libc::c_void);
      return foreign_result != 0;
    }
  }

  /// Converts a Color to a hash value.
  ///
  /// This can be used in hash tables.
  ///
  /// _Since 1.0_
  pub fn hash(&self) -> i32 {
    unsafe {
      let foreign_result = clutter_color_hash(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  /// Retrieves a static color for the given color name.
  ///
  /// Static colors are created by Clutter and are guaranteed to always be
  /// available and valid. They should never be mutated.
  ///
  /// _Since 1.6_
  pub fn get_static(static_color: StaticColor) -> Color {
    unsafe {
      let foreign_result = clutter_color_get_static(static_color);
      return foreign_result;
    }
  }

  /// Parses a string definition of a color, filling the `red`, `green`, `blue`
  /// and `alpha` channels of color.
  ///
  /// The color is not allocated.
  ///
  /// The format of `str` can be either one of:
  ///
  /// - a standard name (as taken from the X11 rgb.txt file)
  /// - a hex value in the form: `#rgb`, `#rrggbb`, `#rgba`, or `#rrggbbaa`
  /// - a RGB color in the form: `rgb(r, g, b)`
  /// - a RGBA color in the form: `rgba(r, g, b, a)`
  /// - a HSL color in the form: `hsl(h, s, l)`
  /// - a HSLA color in the form: `hsla(h, s, l, a)`
  ///
  /// where `r`, `g`, `b` and `a` are (respectively) the red, green, blue color
  /// intensities and the opacity. The `h`, `s` and `l` are (respectively) the
  /// hue, saturation and luminance values.
  ///
  /// In the `rgb()` and `rgba()` formats, the `r`, `g`, and `b` values are
  /// either integers between 0 and 255, or percentage values in the range
  /// between 0% and 100%; the percentages require the `%` character. The `a`
  /// value, if specified, can only be a floating point value between 0.0 and
  /// 1.0.
  ///
  /// In the `hls()` and `hlsa()` formats, the `h` value (hue) is an angle
  /// between 0 and 360.0 degrees; the `l` and `s` values (luminance and
  /// saturation) are percentage values in the range between 0% and 100%. The
  /// `a` value, if specified, can only be a floating point value between 0.0
  /// and 1.0.
  ///
  /// Whitespace inside the definitions is ignored; no leading whitespace is
  /// allowed.
  ///
  /// If the alpha component is not specified then it is assumed to be set to
  /// be fully opaque.
  ///
  /// _Since 1.0_
  pub fn from_string(&mut self, name: &str) -> bool {
    unsafe {
      use std::c_str::ToCStr;
      let foreign_result = clutter_color_from_string(self.opaque, name.to_c_str().unwrap() as *mut i8);
      return foreign_result != 0;
    }
  }

  /// Returns a textual specification of color in the hexadecimal form `#rrggbbaa`
  ///
  /// _Since 0.2_
  pub fn to_string(&self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_color_to_string(self.opaque as *mut libc::c_void);
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  /// Converts a color expressed in HLS (hue, luminance and saturation) values
  /// into a Color.
  pub fn from_hls(&mut self, hue: f32, luminance: f32, saturation: f32) {
    unsafe {
      clutter_color_from_hls(self.opaque, hue, luminance, saturation);
    }
  }

  /// Converts `color` to the HLS format.
  ///
  /// The hue value is in the 0..360 range. The luminance and saturation values
  /// are in the 0..1 range.
  pub fn to_hls(&self) -> (f32, f32, f32) {
    unsafe {
      let mut hue:f32 =std::intrinsics::init();
      let mut luminance:f32 =std::intrinsics::init();
      let mut saturation:f32 =std::intrinsics::init();
      clutter_color_to_hls(self.opaque as *mut libc::c_void, &mut hue, &mut luminance, &mut saturation);
      return (hue, luminance, saturation);
    }
  }

  /// Converts `pixel` from the packed 32-bit integer representation of a four
  /// 8-bit channel color to a Color.
  pub fn from_pixel(&mut self, pixel: i32) {
    unsafe {
      clutter_color_from_pixel(self.opaque, pixel);
    }
  }

  /// Converts `color` into a packed 32-bit integer, containing all four 8-bit
  /// channels used by Color.
  pub fn to_pixel(&self) -> i32 {
    unsafe {
      let foreign_result = clutter_color_to_pixel(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  /// Adds the `other` color to this one and returns the result.
  ///
  /// The alpha channel of result is set as as the maximum value between the
  /// alpha channels of this color and the `other`.
  pub fn add(&self, other: &Color) -> Color {
    unsafe {
      let foreign_result = clutter_color_add(self.opaque as *mut libc::c_void, other.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  /// Subtracts the `other` color from this one and returns the result.
  ///
  /// This method assumes that the components of this color are greater than
  /// the components of the `other`; the result is, otherwise, undefined.
  ///
  /// The alpha channel of the result is set as the minimum value between the
  /// alpha channels of this color and the `other`.
  pub fn subtract(&self, other: &Color) -> Color {
    unsafe {
      let foreign_result = clutter_color_subtract(self.opaque as *mut libc::c_void, other.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  /// Lightens the color by a fixed amount, and returns the result.
  pub fn lighten(&self) -> Color {
    unsafe {
      let foreign_result = clutter_color_lighten(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  /// Darkens the color by a fixed amount, and returns the result.
  pub fn darken(&self) -> Color {
    unsafe {
      let foreign_result = clutter_color_darken(self.opaque as *mut libc::c_void);
      return foreign_result;
    }
  }

  /// Shades the color by `factor`, and returns the result.
  pub fn shade(&self, factor: f64) -> Color {
    unsafe {
      let foreign_result = clutter_color_shade(self.opaque as *mut libc::c_void, factor);
      return foreign_result;
    }
  }

  /// Interpolates between this color and the `final` color using `progress`.
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
  fn clutter_color_get_static(static_color: StaticColor) -> Color;
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
  /// Makes a copy of the color structure.
  ///
  /// _Since 0.2_
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

impl std::ops::Drop for Color {
  /// Frees a color.
  ///
  /// _Since 0.2_
  fn drop(&mut self) {
    unsafe {
      clutter_color_free(self.opaque);
    }
  }
}

extern {
  fn clutter_color_free(self_value: *mut libc::c_void);
}

