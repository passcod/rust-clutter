/// Flags passed to the `Actor#allocate()` method.
///
/// _Since 1.0_
#[repr(i32)]
pub enum Flags {
  /// No flag set
  None = 0,

  /// Whether the absolute origin of the actor has changed; this implies that
  /// any ancestor of the actor has been moved.
  AbsoluteOriginChanged = 2,

  /// Whether the allocation should be delegated to the LayoutManager instance
  /// stored inside the `layout-manager` property of Actor. This flag should
  /// only be used if you are subclassing (FIXME: Rust equivalent?) Actor and
  /// overriding the `.allocate()` method, but you wish to use the default
  /// implementation of the method inside Actor.
  DelegateLayout = 4
}
