use libc;
use super::actor::Actor;
use super::actor::ActorMeta;

pub mod bind;

#[repr(C)]
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

#[repr(C)]
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
