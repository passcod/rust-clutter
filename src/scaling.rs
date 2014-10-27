#![stable]

/// The scaling filters to be used with the `minification-filter` and
/// `magnification-filter` properties.
#[repr(i32)]
pub enum Filter {
  /// Linear interpolation filter
  Linear = 0,

  /// Nearest neighbor interpolation filter
  Nearest = 1,

  /// Trilinear minification filter, with mipmap generation; this filter
  /// linearly interpolates on every axis, as well as between mipmap levels.
  Trilinear = 2
}
