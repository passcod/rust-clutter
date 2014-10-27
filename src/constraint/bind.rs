#![stable]

/// Specifies which property should be used in a binding.
///
/// _Since 1.4_
#[repr(i32)]
pub enum Coordinate {
  /// Bind the X coordinate
  X = 0,

  /// Bind the Y coordinate
  Y = 1,

  /// Bind the width
  Width = 2,

  /// Bind the height
  Height = 3,

  /// Equivalent to X and Y
  ///
  /// _Since 1.6_
  Position = 4,

  /// Equivalent to Width and Height
  ///
  /// _Since 1.6_
  Size = 5,

  /// Equivalent to Position and Size
  ///
  /// _Since 1.10_
  All = 6
}
