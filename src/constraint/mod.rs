#![stable]

use libc;
use super::actor::Actor;
use super::actor::ActorMeta;

pub mod bind;

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct ConstraintRef {
  opaque: *mut libc::c_void
}

/// Trait for constraints on position or size.
///
/// _Since 1.4_
pub trait Constraint {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
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

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct BindConstraintRef {
  opaque: *mut libc::c_void
}

impl BindConstraintRef {
  /// Creates a new constraint, binding an Actor's position to the given
  /// coordinate of the position of source.
  ///
  /// _Since 1.4_
  pub fn new<T: Actor>(source: &mut T, coordinate: bind::Coordinate, offset: f32) -> BindConstraintRef {
    unsafe {
      let foreign_result = clutter_bind_constraint_new(source.as_actor(), coordinate, offset);
      return foreign_result;
    }
  }
}

/// A constraint binding the position or size of an actor.
///
/// BindConstraint is a Constraint that binds the position or the size of the
/// Actor to which it is applied to the position or the size of another Actor,
/// or "source".
///
/// An offset can be applied to the constraint, to avoid overlapping.
/// The offset can also be animated.
///
/// _Since 1.4_
pub trait BindConstraint {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
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
