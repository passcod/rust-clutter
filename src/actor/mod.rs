#![stable]

use libc;
use std;
use cairo;
use super::content::Content;
use super::constraint::Constraint;

pub mod allocation;

#[repr(i32)]
pub enum Flags {
  Mapped = 2,
  Realized = 4,
  Reactive = 8,
  Visible = 16,
  NoLayout = 32
}

#[repr(C)]
pub struct Box {
  x1: f32,
  y1: f32,
  x2: f32,
  y2: f32
}

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct ActorRef {
  opaque: *mut libc::c_void
}

impl ActorRef {
  /// Creates a new Clutter Actor.
  ///
  /// A newly created actor has a floating reference, which will be sunk when it
  /// is added to another actor.
  ///
  /// _Since 1.10_
  pub fn new() -> ActorRef {
    unsafe {
      let foreign_result = clutter_actor_new();
      return foreign_result;
    }
  }
}

/// The basic element of the scene graph.
pub trait Actor {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
  fn as_actor(&self) -> *mut libc::c_void;

  /// Sets flags on the actor.
  ///
  /// This function will emit notifications for the changed properties.
  ///
  /// _Since 1.0_
  fn set_flags(&mut self, flags: Flags) {
    unsafe {
      clutter_actor_set_flags(self.as_actor(), flags);
    }
  }

  /// Unsets flags on the actor.
  ///
  /// This function will emit notifications for the changed properties.
  ///
  /// _Since 1.0_
  fn unset_flags(&mut self, flags: Flags) {
    unsafe {
      clutter_actor_unset_flags(self.as_actor(), flags);
    }
  }

  /// Retrieves the flags set on the actor.
  ///
  /// _Since 1.0_
  fn get_flags(&mut self) -> Flags {
    unsafe {
      let foreign_result = clutter_actor_get_flags(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets the name of the actor.
  ///
  /// The name can be used to identify a Clutter Actor.
  fn set_name(&mut self, name: &str) {
    unsafe {
      use std::c_str::ToCStr;
      clutter_actor_set_name(self.as_actor(), name.to_c_str().unwrap() as *mut i8);
    }
  }

  /// Gets the name of the actor.
  ///
  /// The name can be used to identify a Clutter Actor.
  fn get_name(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_actor_get_name(self.as_actor());
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  /// Retrieves the globally unique id of the actor.
  ///
  /// This method has been deprecated since version 1.8 and should not be used
  /// in newly-written code. The id is not used any longer.
  ///
  /// _Since 0.6_
  #[deprecated]
  fn get_gid(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_actor_get_gid(self.as_actor());
      return foreign_result;
    }
  }

  /// Flags an actor to be displayed.
  ///
  /// An actor that isn't shown will not be rendered on the stage.
  ///
  /// Actors are visible by default.
  ///
  /// If this function is called on an actor without a parent, the
  /// `show-on-set-parent` property will be set to __true__ as a side effect.
  fn show(&mut self) {
    unsafe {
      clutter_actor_show(self.as_actor());
    }
  }

  /// Calls `.show()` on all children of the actor (if any).
  ///
  /// This method has been deprecated since version 1.10 and should not be used
  /// in newly-written code. Actors are visible by default.
  ///
  /// _Since 0.2_
  #[deprecated]
  fn show_all(&mut self) {
    unsafe {
      clutter_actor_show_all(self.as_actor());
    }
  }

  /// Flags an actor to be hidden.
  ///
  /// A hidden actor will not be rendered on the stage.
  ///
  /// Actors are visible by default.
  ///
  /// If this function is called on an actor without a parent, the
  /// `show-on-set-parent` property will be set to __false__ as a side-effect.
  fn hide(&mut self) {
    unsafe {
      clutter_actor_hide(self.as_actor());
    }
  }

  /// Calls `.hide()` on all child actors (if any).
  ///
  /// This method has been deprecated since version 1.10 and should not be used
  /// in newly-written code. Using `clutter_actor_hide()` on the actor will
  /// prevent its children from being painted as well.
  ///
  /// _Since 0.2_
  #[deprecated]
  fn hide_all(&mut self) {
    unsafe {
      clutter_actor_hide_all(self.as_actor());
    }
  }

  /// Informs the actor that it is attached to a stage.
  ///
  /// It can use this to allocate resources if it wanted to delay allocation
  /// until it would be rendered. However it is perfectly acceptable for an
  /// actor to create resources before being realized because Clutter only ever
  /// has a single rendering context so that actor is free to be moved from one
  /// stage to another.
  ///
  /// This function does nothing if the actor is already realized.
  ///
  /// Because a realized actor must have realized parent actors, calling
  /// `.realize()` will also realize all parents of the actor.
  ///
  /// This function does not realize child actors, except in the special case
  /// that realizing the stage, when the stage is visible, will suddenly map
  /// (and thus realize) the children of the stage.
  ///
  /// This method has been deprecated since version 1.16 and should not be used
  /// in newly-written code. Actors are automatically realized, and nothing
  /// requires explicit realization.
  #[deprecated]
  fn realize(&mut self) {
    unsafe {
      clutter_actor_realize(self.as_actor());
    }
  }

  /// Informs the actor that it may be being destroyed / moved to another stage.
  ///
  /// The actor may want to destroy any underlying graphics resources at this
  /// point. However it is perfectly acceptable for it to retain the resources
  /// until the actor is destroyed because Clutter only ever uses a single
  /// rendering context and all of the graphics resources are valid on any stage.
  ///
  /// Because mapped actors must be realized, actors may not be unrealized if
  /// they are mapped. This function hides the actor to be sure it isn't mapped,
  /// an application-visible side effect that you may not be expecting.
  ///
  /// This function should not be called by application code.
  ///
  /// This function should not really be in the public API, because there isn't
  /// a good reason to call it. Actor will already unrealize things for you when
  /// it's important to do so.
  ///
  /// If you were using `.unrealize()` in a dispose implementation, then don't,
  /// just chain up to Actor's `dispose`.
  ///
  /// If you were using `.unrealize()` to implement unrealizing children of your
  /// container, then don't, Actor will already take care of that.
  ///
  /// This method has been deprecated since version 1.16 and should not be used
  /// in newly-written code. Actors are automatically unrealized, and nothing
  /// requires explicit realization.
  #[deprecated]
  fn unrealize(&mut self) {
    unsafe {
      clutter_actor_unrealize(self.as_actor());
    }
  }

  /// Renders the actor to display.
  ///
  /// This method should not be called directly by applications. Call
  /// `.queue_redraw()` to queue paints, instead.
  ///
  /// It is context-aware, and will either cause a regular paint or
  /// a pick paint. It will emit the `paint` signal or the `pick` signal,
  /// depending on the context. It will not paint the actor if the actor is set
  /// to 0, unless it is performing a pick paint.
  fn paint(&mut self) {
    unsafe {
      clutter_actor_paint(self.as_actor());
    }
  }

  /// Run the next stage of the paint sequence.
  ///
  /// This function should only be called within the implementation of the
  /// `run` virtual of a Clutter::Effect. It will cause the `run()` method of
  /// the next effect to be applied, or it will paint the actual actor if the
  /// current effect is the last effect in the chain.
  ///
  /// _Since 1.8_
  fn continue_paint(&mut self) {
    unsafe {
      clutter_actor_continue_paint(self.as_actor());
    }
  }

  /// Queues up a redraw of the actor and any children.
  ///
  /// The redraw occurs once the main loop becomes idle (after the current batch
  /// of events has been processed, roughly).
  ///
  /// Applications rarely need to call this, as redraws are handled
  /// automatically by modification functions.
  ///
  /// This method will not do anything if the actor is not visible, or if the
  /// actor is inside an invisible part of the scenegraph. Also be aware that
  /// painting is a noop for actors with an opacity of 0.
  ///
  /// When you are implementing a custom actor you must queue a redraw whenever
  /// some private state changes that will affect painting or picking of your
  /// actor.
  fn queue_redraw(&mut self) {
    unsafe {
      clutter_actor_queue_redraw(self.as_actor());
    }
  }

  /// Queues a redraw on the actor limited to a specific, actor-relative
  /// rectangular area.
  ///
  /// _Since 1.10_
  fn queue_redraw_with_clip(&mut self, clip: &mut cairo::region::Rectangle) {
    unsafe {
      clutter_actor_queue_redraw_with_clip(self.as_actor(), clip);
    }
  }

  /// Indicates that the actor's size request or other layout-affecting
  /// properties may have changed.
  ///
  /// This function is used inside Actor subclass implementations, not by
  /// applications directly. Queueing a new layout automatically queues a
  /// redraw as well.
  ///
  /// _Since 0.8_
  fn queue_relayout(&mut self) {
    unsafe {
      clutter_actor_queue_relayout(self.as_actor());
    }
  }

  /// Checks whether the actor should paint itself in pick mode or not.
  ///
  /// Should be called inside the implementation of the `pick` virtual function.
  ///
  /// This method should never be called directly by applications.
  fn should_pick_repaint(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_should_pick_repaint(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets the CLUTTER_ACTOR_MAPPED flag on the actor and possibly maps and
  /// realizes its children if they are visible.
  ///
  /// Does nothing if the actor is not visible.
  ///
  /// Calling this method is strongly disencouraged: the default
  /// implementation of `.map()` will map all the children of an actor when
  /// mapping its parent.
  ///
  /// When overriding map, it is mandatory to chain up to the parent
  /// implementation.
  ///
  /// _Since 1.0_
  // FIXME: Not sure what the doc means here
  fn map(&mut self) {
    unsafe {
      clutter_actor_map(self.as_actor());
    }
  }

  /// Unsets the CLUTTER_ACTOR_MAPPED flag on the actor and possibly unmaps its
  /// children if they were mapped.
  ///
  /// Calling this method is not encouraged: the default Actor implementation
  /// of `.unmap()` will also unmap any eventual children by default when their
  /// parent is unmapped.
  ///
  /// When overriding `.unmap()`, it is mandatory to chain up to the parent
  /// implementation.
  ///
  /// It is important to note that the implementation of the `.unmap()` virtual
  /// function may be called after the `.destroy()` or the
  /// `GObjectClass#dispose()` implementation, but it is guaranteed to be called
  /// before the `GObjectClass#finalize()` implementation.
  ///
  /// _Since 1.0_
  // FIXME: Not sure what the doc means here
  fn unmap(&mut self) {
    unsafe {
      clutter_actor_unmap(self.as_actor());
    }
  }

  /// Asks the actor's implementation whether it may contain overlapping
  /// primitives.
  ///
  /// For example; Clutter may use this to determine whether the painting
  /// should be redirected to an offscreen buffer to correctly implement the
  /// opacity property.
  ///
  /// Custom actors can override the default response by implementing the
  /// `.has_overlaps()` virtual function. See `.set_offscreen_redirect()` for
  /// more information.
  ///
  /// _Since 1.8_
  fn has_overlaps(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_has_overlaps(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Returns whether an Actor has any mapped clones.
  ///
  /// _Since 1.16_
  fn has_mapped_clones(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_has_mapped_clones(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Computes the requested minimum and natural widths for an actor,
  /// optionally depending on the specified height, or if they are already
  /// computed, returns the cached values.
  ///
  /// An actor may not get its request — depending on the layout manager
  /// that's in effect.
  ///
  /// A request should not incorporate the actor's scale or anchor point;
  /// those transformations do not affect layout, only rendering.
  ///
  /// _Since 0.8_
  fn get_preferred_width(&mut self, for_height: f32) -> (f32, f32) {
    unsafe {
      let mut min_width_p:f32 = std::intrinsics::init();
      let mut natural_width_p:f32 = std::intrinsics::init();
      clutter_actor_get_preferred_width(self.as_actor(), for_height, &mut min_width_p, &mut natural_width_p);
      return (min_width_p, natural_width_p);
    }
  }

  /// Computes the requested minimum and natural heights for an actor, or if
  /// they are already computed, returns the cached values.
  ///
  /// An actor may not get its request — depending on the layout manager
  /// that's in effect.
  ///
  /// A request should not incorporate the actor's scale or anchor point;
  /// those transformations do not affect layout, only rendering.
  ///
  /// _Since 0.8_
  fn get_preferred_height(&mut self, for_width: f32) -> (f32, f32) {
    unsafe {
      let mut min_height_p:f32 = std::intrinsics::init();
      let mut natural_height_p:f32 = std::intrinsics::init();
      clutter_actor_get_preferred_height(self.as_actor(), for_width, &mut min_height_p, &mut natural_height_p);
      return (min_height_p, natural_height_p);
    }
  }

  /// Sets whether an actor has a fixed position set (and will thus be
  /// unaffected by any layout manager).
  ///
  /// _Since 0.8_
  fn set_fixed_position_set(&mut self, fixed_position_set: bool) {
    unsafe {
      clutter_actor_set_fixed_position_set(self.as_actor(), (fixed_position_set as i32));
    }
  }

  /// Computes the preferred minimum and natural size of an actor, taking into
  /// account the actor's geometry management (either height-for-width or
  /// width-for-height).
  ///
  /// The width and height used to compute the preferred height and preferred
  /// width are the actor's natural ones.
  ///
  /// If you need to control the height for the preferred width, or the width
  /// for the preferred height, you should use `.get_preferred_width()` and
  /// `.get_preferred_height()`, and check the actor's preferred geometry
  /// management using the `request-mode` property.
  ///
  /// _Since 0.8_
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

  /// Checks whether an actor has a fixed position set (and will thus be
  /// unaffected by any layout manager).
  ///
  /// _Since 0.8_
   fn get_fixed_position_set(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_get_fixed_position_set(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Checks if the actor has an up-to-date allocation assigned to it.
  ///
  /// This means that the actor should have an allocation: it's visible and has
  /// a parent. It also means that there is no outstanding relayout request in
  /// progress for the actor or its children (there might be other outstanding
  /// layout requests in progress that will cause the actor to get a new
  /// allocation when the stage is laid out, however).
  ///
  /// If this function returns __false__, then the actor will normally be
  /// allocated before it is next drawn on the screen.
  ///
  /// _Since 1.4_
  fn has_allocation(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_has_allocation(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets whether an Actor should expand horizontally.
  ///
  /// This means that layout manager should allocate extra space for the actor,
  /// if possible.
  ///
  /// Setting an actor to expand will also make all its parent expand, so that
  /// it's possible to build an actor tree and only set this flag on its leaves
  /// and not on every single actor.
  ///
  /// _Since 1.12_
  fn set_x_expand(&mut self, expand: bool) {
    unsafe {
      clutter_actor_set_x_expand(self.as_actor(), (expand as i32));
    }
  }

  /// Retrieves the value set with `.set_x_expand()`.
  ///
  /// See also: `.needs_expand()`.
  ///
  /// _Since 1.12_
  fn get_x_expand(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_get_x_expand(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets whether a ClutterActor should expand horizontally.
  ///
  /// This means that layout manager should allocate extra space for the actor,
  /// if possible.
  ///
  /// Setting an actor to expand will also make all its parent expand, so that
  /// it's possible to build an actor tree and only set this flag on its leaves
  /// and not on every single actor.
  ///
  /// _Since 1.12_
  fn set_y_expand(&mut self, expand: bool) {
    unsafe {
      clutter_actor_set_y_expand(self.as_actor(), (expand as i32));
    }
  }

  /// Retrieves the value set with `.set_y_expand()`.
  ///
  /// See also: `.needs_expand()`.
  ///
  /// _Since 1.12_
  fn get_y_expand(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_get_y_expand(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Checks whether an actor, or any of its children, is set to expand
  /// horizontally or vertically.
  ///
  /// This method should only be called by layout managers that can assign
  /// extra space to their children.
  ///
  /// If you want to know whether the actor was explicitly set to expand, use
  /// `.get_x_expand()` or `.get_y_expand()`.
  ///
  /// _Since 1.12_
  fn needs_expand(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_needs_expand(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets the actor's size request in pixels.
  ///
  /// This overrides any "normal" size request the actor would have. For example
  /// a text actor might normally request the size of the text; this method
  /// would force a specific size instead.
  ///
  /// If width and/or height are -1 the actor will use its "normal" size request
  /// instead of overriding it, i.e. you can "unset" the size with -1.
  ///
  /// This method sets or unsets both the minimum and natural size.
  fn set_size(&mut self, for_width: f32, for_height: f32) {
    unsafe {
      clutter_actor_set_size(self.as_actor(), for_width, for_height);
    }
  }

  /// Tries to "do what you mean" and return the size an actor will have.
  ///
  /// If the actor has a valid allocation, the allocation will be returned;
  /// otherwise, the actors natural size request will be returned.
  ///
  /// If you care whether you get the request vs. the allocation, you should
  /// probably call a different function like `.get_allocation_box()` or
  /// ``get_preferred_width()`.
  ///
  /// _Since 0.2_
  fn get_size(&mut self) -> (f32, f32) {
    unsafe {
      let mut width:f32 = std::intrinsics::init();
      let mut height:f32 = std::intrinsics::init();
      clutter_actor_get_size(self.as_actor(), &mut width, &mut height);
      return (width, height);
    }
  }

  /// Sets the actor's fixed position in pixels relative to any parent actor.
  ///
  /// If a layout manager is in use, this position will override the layout
  /// manager and force a fixed position.
  fn set_position(&mut self, x: f32, y: f32) {
    unsafe {
      clutter_actor_set_position(self.as_actor(), x, y);
    }
  }

  /// This function tries to "do what you mean" and tell you where the actor
  /// is, prior to any transformations.
  ///
  /// Retrieves the fixed position of an actor in pixels, if one has been set;
  /// otherwise, if the allocation is valid, returns the actor's allocated
  /// position; otherwise, returns (0, 0).
  ///
  /// The returned position is in pixels.
  ///
  /// _Since 0.6_
  fn get_position(&mut self) -> (f32, f32) {
    unsafe {
      let mut x:f32 = std::intrinsics::init();
      let mut y:f32 = std::intrinsics::init();
      clutter_actor_get_position(self.as_actor(), &mut x, &mut y);
      return (x, y);
    }
  }

  /// Forces a width on an actor, causing the actor's preferred width and height
  /// (if any) to be ignored.
  ///
  /// If width is -1 the actor will use its preferred width request instead of
  /// overriding it, i.e. you can "unset" the width with -1.
  ///
  /// This method sets both the minimum and natural size of the actor.
  ///
  /// _Since 0.2_
  fn set_width(&mut self, width: f32) {
    unsafe {
      clutter_actor_set_width(self.as_actor(), width);
    }
  }

  /// Retrieves the width of an Actor.
  ///
  /// If the actor has a valid allocation, this method will return the width of
  /// the allocated area given to the actor.
  ///
  /// If the actor does not have a valid allocation, this method will return the
  /// actor's natural width, that is the preferred width of the actor.
  ///
  /// If you care whether you get the preferred width or the width that has been
  /// assigned to the actor, you should probably call a different method like
  /// `.get_allocation_box()` to retrieve the allocated size or
  /// `.get_preferred_width()` to retrieve the preferred width.
  ///
  /// If an actor has a fixed width, for instance a width that has been assigned
  /// using `.set_width()`, the width returned will be the same value.
  fn get_width(&mut self) -> f32 {
    unsafe {
      let foreign_result = clutter_actor_get_width(self.as_actor());
      return foreign_result;
    }
  }

  /// Forces a height on an actor, causing the actor's preferred width and
  /// height (if any) to be ignored.
  ///
  /// If height is -1 the actor will use its preferred height instead of
  /// overriding it, i.e. you can "unset" the height with -1.
  ///
  /// This method sets both the minimum and natural size of the actor.
  ///
  /// _Since 0.2_
  fn set_height(&mut self, height: f32) {
    unsafe {
      clutter_actor_set_height(self.as_actor(), height);
    }
  }

  /// Retrieves the height of an Actor.
  ///
  /// If the actor has a valid allocation, this method will return the height
  /// of the allocated area given to the actor.
  ///
  /// If the actor does not have a valid allocation, this method will return
  /// the actor's natural height, that is the preferred height of the actor.
  ///
  /// If you care whether you get the preferred height or the height that has
  /// been assigned to the actor, you should probably call a different method
  /// like `.get_allocation_box()` to retrieve the allocated size or
  /// `.get_preferred_height()` to retrieve the preferred height.
  ///
  /// If an actor has a fixed height, for instance a height that has been
  /// assigned using `.set_height()`, the height returned will be the same
  /// value.
  fn get_height(&mut self) -> f32 {
    unsafe {
      let foreign_result = clutter_actor_get_height(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets the actor's X coordinate, relative to its parent, in pixels.
  ///
  /// Overrides any layout manager and forces a fixed position for the actor.
  ///
  /// The `x` property is animatable.
  ///
  /// _Since 0.6_
  fn set_x(&mut self, x: f32) {
    unsafe {
      clutter_actor_set_x(self.as_actor(), x);
    }
  }

  /// Retrieves the X coordinate of an Actor.
  ///
  /// This method tries to "do what you mean", by returning the correct value
  /// depending on the actor's state.
  ///
  /// If the actor has a valid allocation, this method will return the X
  /// coordinate of the origin of the allocation box.
  ///
  /// If the actor has any fixed coordinate set using `.set_x()`,
  /// `.set_position()` or `.set_geometry()`, this method will return that
  /// coordinate.
  ///
  /// If both the allocation and a fixed position are missing, this method will
  /// return 0.
  fn get_x(&mut self) -> f32 {
    unsafe {
      let foreign_result = clutter_actor_get_x(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets the actor's Y coordinate, relative to its parent, in pixels.
  ///
  /// Overrides any layout manager and forces a fixed position for the actor.
  ///
  /// The `y` property is animatable.
  ///
  /// _Since 0.6_
  fn set_y(&mut self, y: f32) {
    unsafe {
      clutter_actor_set_y(self.as_actor(), y);
    }
  }

  /// Retrieves the Y coordinate of an Actor.
  ///
  /// This method tries to "do what you mean", by returning the correct value
  /// depending on the actor's state.
  ///
  /// If the actor has a valid allocation, this method will return the Y
  /// coordinate of the origin of the allocation box.
  ///
  /// If the actor has any fixed coordinate set using `.set_y()`,
  /// `.set_position()` or `.set_geometry()`, this method will return that
  /// coordinate.
  ///
  /// If both the allocation and a fixed position are missing, this method will
  /// return 0.
  fn get_y(&mut self) -> f32 {
    unsafe {
      let foreign_result = clutter_actor_get_y(self.as_actor());
      return foreign_result;
    }
  }

  /// Moves an actor by the specified distance relative to its current position
  /// in pixels.
  ///
  /// This method modifies the fixed position of an actor and thus removes it
  /// from any layout management. Another way to move an actor is with an anchor
  /// point, see `.set_anchor_point()`, or with an additional translation, using
  /// `.set_translation()`.
  ///
  /// _Since 0.2_
  fn move_by(&mut self, dx: f32, dy: f32) {
    unsafe {
      clutter_actor_move_by(self.as_actor(), dx, dy);
    }
  }

  /// Sets the actor's position on the Z axis.
  ///
  /// _Since 1.12_
  fn set_z_position(&mut self, y: f32) {
    unsafe {
      clutter_actor_set_z_position(self.as_actor(), y);
    }
  }

  /// Retrieves the actor's position on the Z axis.
  ///
  /// _Since 1.12_
  fn get_z_position(&mut self) -> f32 {
    unsafe {
      let foreign_result = clutter_actor_get_z_position(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets the position of the `pivot-point` around which the scaling and
  /// rotation transformations occur.
  ///
  /// The pivot point's coordinates are in normalized space, with the
  /// (0, 0) point being the top left corner of the actor, and the (1, 1)
  /// point being the bottom right corner.
  ///
  /// _Since 1.12_
  fn set_pivot_point(&mut self, pivot_x: f32, pivot_y: f32) {
    unsafe {
      clutter_actor_set_pivot_point(self.as_actor(), pivot_x, pivot_y);
    }
  }

  /// Retrieves the coordinates of the `pivot-point`.
  ///
  /// _Since 1.12_
  fn get_pivot_point(&mut self) -> (f32, f32) {
    unsafe {
      let mut pivot_x:f32 = std::intrinsics::init();
      let mut pivot_y:f32 = std::intrinsics::init();
      clutter_actor_get_pivot_point(self.as_actor(), &mut pivot_x, &mut pivot_y);
      return (pivot_x, pivot_y);
    }
  }

  /// Sets the component on the Z axis of the `pivot-point` around which the
  /// scaling and rotation transformations occur.
  ///
  /// The `pivot_z` value is expressed as a distance along the Z axis.
  ///
  /// _SInce 1.12_
  fn set_pivot_point_z(&mut self, pivot_z: f32) {
    unsafe {
      clutter_actor_set_pivot_point_z(self.as_actor(), pivot_z);
    }
  }

  /// Retrieves the Z component of the `pivot-point`.
  ///
  /// _Since 1.12_
  fn get_pivot_point_z(&mut self) -> f32 {
    unsafe {
      let foreign_result = clutter_actor_get_pivot_point_z(self.as_actor());
      return foreign_result;
    }
  }

  /// Scales an actor with the given factors.
  ///
  /// The scale transformation is relative the the `pivot-point`.
  ///
  /// The `scale-x` and `scale-y` properties are animatable.
  ///
  /// _Since 0.2_
  fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
    unsafe {
      clutter_actor_set_scale(self.as_actor(), scale_x, scale_y);
    }
  }

  /// Retrieves an actor's scale factors.
  ///
  /// _Since 0.2_
  fn get_scale(&mut self) -> (f32, f32) {
    unsafe {
      let mut scale_x:f32 = std::intrinsics::init();
      let mut scale_y:f32 = std::intrinsics::init();
      clutter_actor_get_scale(self.as_actor(), &mut scale_x, &mut scale_y);
      return (scale_x, scale_y);
    }
  }

  /// Scales an actor on the Z axis by the given `scale_z` factor.
  ///
  /// The scale transformation is relative to the `pivot-point`.
  ///
  /// The `scale-z` property is animatable.
  ///
  /// _Since 1.12_
  fn set_scale_z(&mut self, pivot_z: f32) {
    unsafe {
      clutter_actor_set_scale_z(self.as_actor(), pivot_z);
    }
  }

  /// Retrieves the scaling factor along the Z axis, as set using
  /// `.get_scale_z()`.
  ///
  /// _Since 1.12_
  fn get_scale_z(&mut self) -> f32 {
    unsafe {
      let foreign_result = clutter_actor_get_scale_z(self.as_actor());
      return foreign_result;
    }
  }

  /// Checks whether the actor is scaled in either dimension.
  ///
  /// _Since 0.6_
  fn is_scaled(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_is_scaled(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Checks whether any rotation is applied to the actor.
  ///
  /// _Since 0.6_
  fn is_rotated(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_is_rotated(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets an additional translation transformation on the actor, relative to
  /// the `pivot-point`.
  ///
  /// _Since 1.12_
  fn set_translation(&mut self, translate_x: f32, translate_y: f32, translate_z: f32) {
    unsafe {
      clutter_actor_set_translation(self.as_actor(), translate_x, translate_y, translate_z);
    }
  }

  /// Retrieves the translation set using `.set_translation()`.
  ///
  /// _Since 1.12_
  fn get_translation(&mut self) -> (f32, f32, f32) {
    unsafe {
      let mut translate_x:f32 = std::intrinsics::init();
      let mut translate_y:f32 = std::intrinsics::init();
      let mut translate_z:f32 = std::intrinsics::init();
      clutter_actor_get_translation(self.as_actor(), &mut translate_x, &mut translate_y, &mut translate_z);
      return (translate_x, translate_y, translate_z);
    }
  }

  /// Transforms point in coordinates relative to the actor into
  /// screen-relative coordinates with the current actor transformation
  /// (i.e. scale, rotation, etc).
  ///
  /// _Since 0.4_
  fn apply_transform_to_point(&mut self, x: f32, y: f32) -> (bool, f32, f32) {
    unsafe {
      let mut x_out:f32 = std::intrinsics::init();
      let mut y_out:f32 = std::intrinsics::init();
      let foreign_result = clutter_actor_apply_transform_to_point(self.as_actor(), x, y, &mut x_out, &mut y_out);
      return (foreign_result != 0, x_out, y_out);
    }
  }

  /// Translates screen coordinates (x, y) to coordinates relative to the actor.
  /// 
  /// For example, it can be used to translate screen events from global screen
  /// coordinates into actor-local coordinates.
  ///
  /// The conversion can fail, notably if the transform stack results in the
  /// actor being projected on the screen as a mere line.
  ///
  /// The conversion should not be expected to be pixel-perfect due to the
  /// nature of the operation. In general the error grows when the skewing of
  /// the actor rectangle on screen increases.
  ///
  /// This function can be computationally intensive.
  ///
  /// This function only works when the allocation is up-to-date, i.e. inside
  /// of the `.paint()` implementation.
  ///
  /// _Since 0.6_
  fn transform_stage_point(&mut self, x: f32, y: f32) -> (bool, f32, f32) {
    unsafe {
      let mut x_out:f32 = std::intrinsics::init();
      let mut y_out:f32 = std::intrinsics::init();
      let foreign_result = clutter_actor_transform_stage_point(self.as_actor(), x, y, &mut x_out, &mut y_out);
      return (foreign_result != 0, x_out, y_out);
    }
  }

  /// Gets the absolute position of an actor, in pixels relative to the stage.
  ///
  /// _Since 0.8_
  fn get_transformed_position(&mut self) -> (f32, f32) {
    unsafe {
      let mut x:f32 = std::intrinsics::init();
      let mut y:f32 = std::intrinsics::init();
      clutter_actor_get_transformed_position(self.as_actor(), &mut x, &mut y);
      return (x, y);
    }
  }

  /// Gets the absolute size of an actor in pixels, taking into account the
  /// scaling factors.
  ///
  /// If the actor has a valid allocation, the allocated size will be used. If
  /// the actor doesn't have a valid allocation then the preferred size will be
  /// transformed and returned.
  ///
  /// If you want the transformed allocation, see
  /// `.get_abs_allocation_vertices()` instead.
  ///
  /// When the actor (or one of its ancestors) is rotated around the X or Y
  /// axis, it no longer appears as on the stage as a rectangle, but as a
  /// generic quadrangle; in that case this function returns the size of the
  /// smallest rectangle that encapsulates the entire quad. Please note that
  /// in this case no assumptions can be made about the relative position of
  /// this envelope to the absolute position of the actor, as returned by
  /// `.get_transformed_position()`; if you need this information, you need to
  /// use `.get_abs_allocation_vertices()` to get the coords of the actual
  /// quadrangle.
  ///
  /// _Since 0.8_
  fn get_transformed_size(&mut self) -> (f32, f32) {
    unsafe {
      let mut width:f32 = std::intrinsics::init();
      let mut height:f32 = std::intrinsics::init();
      clutter_actor_get_transformed_size(self.as_actor(), &mut width, &mut height);
      return (width, height);
    }
  }

  /// Retrieves the absolute opacity of the actor, as it appears on the stage.
  ///
  /// This function traverses the hierarchy chain and composites the opacity of
  /// the actor with that of its parents.
  ///
  /// This function is intended for subclasses to use in the paint virtual
  /// function, to paint themselves with the correct opacity.
  ///
  /// _Since 0.8_
  fn get_paint_opacity(&mut self) -> i8 {
    unsafe {
      let foreign_result = clutter_actor_get_paint_opacity(self.as_actor());
      return foreign_result;
    }
  }

  /// Retrieves the 'paint' visibility of an actor recursively checking for non
  /// visible parents.
  ///
  /// This is by definition the same as CLUTTER_ACTOR_IS_MAPPED.
  ///
  /// _Since 0.8.4_
  fn get_paint_visibility(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_get_paint_visibility(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets the contents of an Actor.
  ///
  /// _Since 1.10_
  fn set_content<T: Content>(&mut self, content: &mut T) {
    unsafe {
      clutter_actor_set_content(self.as_actor(), content.as_content());
    }
  }

  /// Retrieves the contents of an Actor.
  ///
  /// _Since 1.10_
  fn get_content(&mut self) -> super::content::ContentRef {
    unsafe {
      let foreign_result = clutter_actor_get_content(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets the minification and magnification filter to be applied when scaling
  /// the 'content' of an Actor.
  ///
  /// The `minification-filter` will be used when reducing the size of the
  /// content; the `magnification-filter` will be used when increasing the size
  /// of the content.
  ///
  /// _Since 1.10_
  fn set_content_scaling_filters(&mut self, min: super::scaling::Filter, mag: super::scaling::Filter) {
    unsafe {
      clutter_actor_set_content_scaling_filters(self.as_actor(), min, mag);
    }
  }

  /// Retrieves the values set using `.set_content_scaling_filters()`.
  ///
  /// _Since 1.10_
  fn get_content_scaling_filters(&mut self) -> (super::scaling::Filter, super::scaling::Filter) {
    unsafe {
      let mut min:super::scaling::Filter = std::intrinsics::init();
      let mut mag:super::scaling::Filter = std::intrinsics::init();
      clutter_actor_get_content_scaling_filters(self.as_actor(), &mut min, &mut mag);
      return (min, mag);
    }
  }

  /// Sets clip area for the actor.
  ///
  /// The clip area is always computed from the upper left corner of the actor,
  /// even if the anchor point is set otherwise.
  ///
  /// _Since 0.6_
  fn set_clip(&mut self, xoff: f32, yoff: f32, width: f32, height: f32) {
    unsafe {
      clutter_actor_set_clip(self.as_actor(), xoff, yoff, width, height);
    }
  }

  /// Removes clip area from the actor.
  fn remove_clip(&mut self) {
    unsafe {
      clutter_actor_remove_clip(self.as_actor());
    }
  }

  /// Determines whether the actor has a clip area set or not.
  ///
  /// _Since 0.1.1_
  fn has_clip(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_has_clip(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Gets the clip area for the actor, if any is set.
  ///
  /// _Since 0.6_
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

  /// Sets whether the actor should be clipped to the same size as its
  /// allocation.
  ///
  /// _Since 1.4_
  fn set_clip_to_allocation(&mut self, clip_set: bool) {
    unsafe {
      clutter_actor_set_clip_to_allocation(self.as_actor(), (clip_set as i32));
    }
  }

  /// Retrieves the value set using `.set_clip_to_allocation()`.
  ///
  /// _Since 1.4_
  fn get_clip_to_allocation(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_get_clip_to_allocation(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets the actor's opacity, with zero being completely transparent and
  /// 255 (0xff) being fully opaque.
  ///
  /// The `opacity` property is animatable.
  fn set_opacity(&mut self, opacity: i8) {
    unsafe {
      clutter_actor_set_opacity(self.as_actor(), opacity);
    }
  }

  /// Retrieves the opacity value of an actor, as set by `.set_opacity()`.
  ///
  /// For retrieving the absolute opacity of the actor inside a paint virtual
  /// function, see `.get_paint_opacity()`.
  fn get_opacity(&mut self) -> i8 {
    unsafe {
      let foreign_result = clutter_actor_get_opacity(self.as_actor());
      return foreign_result;
    }
  }

  /// Checks whether the actor is being currently painted by a Clutter Clone.
  ///
  /// This method is useful only inside the `::paint` virtual function
  /// implementations or within handlers for the `paint` signal.
  ///
  /// It should not be used by applications.
  ///
  /// _Since 1.0_
  fn is_in_clone_paint(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_is_in_clone_paint(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Adds child to the children of the actor.
  ///
  /// This method will acquire a reference on `child` that will only be
  /// released when calling `.remove_child()`.
  ///
  /// This method will take into consideration the `depth` of `child`, and will
  /// keep the list of children sorted.
  ///
  /// This method will emit the `actor-added` signal on the actor.
  ///
  /// _Since 1.10_
  fn add_child<T: Actor>(&mut self, child: &mut T) {
    unsafe {
      clutter_actor_add_child(self.as_actor(), child.as_actor());
    }
  }

  /// Inserts child into the list of children of the actor, above another child
  /// of the actor as specified.
  ///
  /// This method has otherwise the same behavior as `.add_child()`.
  ///
  /// _Since 1.10_
  fn insert_child_above<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
    unsafe {
      clutter_actor_insert_child_above(self.as_actor(), child.as_actor(), sibling.as_actor());
    }
  }

  /// Inserts child into the list of children of the actor, using the given
  /// `index`. If `index` is greater than the number of children in the actor,
  /// or is less than 0, then the new child is added at the end.
  ///
  /// This method has otherwise the same behavior as `.add_child()`.
  ///
  /// _Since 1.10_
  fn insert_child_at_index<T: Actor>(&mut self, child: &mut T, index: i32) {
    unsafe {
      clutter_actor_insert_child_at_index(self.as_actor(), child.as_actor(), index);
    }
  }

  /// Inserts child into the list of children of the actor, below another child
  /// of the actor as specified.
  ///
  /// This method has otherwise the same behavior as `.add_child()`.
  ///
  /// _Since 1.10_
  fn insert_child_below<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
    unsafe {
      clutter_actor_insert_child_below(self.as_actor(), child.as_actor(), sibling.as_actor());
    }
  }

  /// Replaces `old_child` with `new_child` in the list of children of the
  /// actor.
  ///
  /// _Since 1.10_
  fn replace_child<T: Actor, U: Actor>(&mut self, old_child: &mut T, new_child: &mut U) {
    unsafe {
      clutter_actor_replace_child(self.as_actor(), old_child.as_actor(), new_child.as_actor());
    }
  }

  /// Removes `child` from the children of the actor.
  ///
  /// This method will release the reference added by `.add_child()` and
  /// similar methods, so if you want to keep using `child` you will have to
  /// acquire a referenced on it before calling this.
  ///
  /// This method will emit the `actor-removed` signal on the actor.
  ///
  /// _Since 1.10_
  fn remove_child<T: Actor>(&mut self, child: &mut T) {
    unsafe {
      clutter_actor_remove_child(self.as_actor(), child.as_actor());
    }
  }

  /// Removes all children of the actor.
  ///
  /// This method releases the reference added by inserting a child actor in
  /// the list of children of the actor.
  ///
  /// If the reference count of a child drops to zero, the child will be
  /// destroyed. If you want to ensure the destruction of all the children of
  /// the actor, use `.destroy_all_children()`.
  ///
  /// _Since 1.10_
  fn remove_all_children(&mut self) {
    unsafe {
      clutter_actor_remove_all_children(self.as_actor());
    }
  }

  /// Destroys all children of the actor.
  ///
  /// This method releases the reference added by inserting a child actor in
  /// the list of children of the actor, and ensures that the `destroy` signal
  /// is emitted on each child of the actor.
  ///
  /// By default, Actor will emit the `destroy` signal when its reference count
  /// drops to 0; the default handler of the `destroy` signal will destroy all
  /// the children of an actor. This method ensures that all children are
  /// destroyed, instead of just removed from the actor, unlike
  /// `.remove_all_children()` which will merely release the reference and
  /// remove each child.
  ///
  /// Unless you acquired an additional reference on each child of self prior to
  /// calling `.remove_all_children()` and want to reuse the actors, you should
  /// use `.destroy_all_children()` in order to make sure that children are
  /// destroyed and signal handlers are disconnected even in cases where circular
  /// references prevent this from automatically happening through reference
  /// counting alone.
  ///
  /// _Since 1.10_
  fn destroy_all_children(&mut self) {
    unsafe {
      clutter_actor_destroy_all_children(self.as_actor());
    }
  }

  /// Retrieves the first child of the actor.
  ///
  /// The returned pointer is only valid until the scene graph changes; it is
  /// not safe to modify the list of children of the actor while iterating it.
  ///
  /// _Since 1.10_
  fn get_first_child(&mut self) -> ActorRef {
    unsafe {
      let foreign_result = clutter_actor_get_first_child(self.as_actor());
      return foreign_result;
    }
  }

  /// Retrieves the sibling of the actor that comes after it in the list of
  /// children of the actor's parent.
  ///
  /// The returned pointer is only valid until the scene graph changes; it is
  /// not safe to modify the list of children of the actor while iterating it.
  ///
  /// _Since 1.10_
  fn get_next_sibling(&mut self) -> ActorRef {
    unsafe {
      let foreign_result = clutter_actor_get_next_sibling(self.as_actor());
      return foreign_result;
    }
  }

  /// Retrieves the sibling of self that comes before it in the list of
  /// children of the actor's parent.
  ///
  /// The returned pointer is only valid until the scene graph changes; it is
  /// not safe to modify the list of children of the actor while iterating it.
  ///
  /// _Since 1.10_
  fn get_previous_sibling(&mut self) -> ActorRef {
    unsafe {
      let foreign_result = clutter_actor_get_previous_sibling(self.as_actor());
      return foreign_result;
    }
  }

  /// Retrieves the last child of the actor.
  ///
  /// The returned pointer is only valid until the scene graph changes; it is
  /// not safe to modify the list of children of the actor while iterating it.
  ///
  /// _Since 1.10_
  fn get_last_child(&mut self) -> ActorRef {
    unsafe {
      let foreign_result = clutter_actor_get_last_child(self.as_actor());
      return foreign_result;
    }
  }

  /// Retrieves the actor at the given `index` inside the list of children of
  /// the actor.
  ///
  /// _Since 1.10_
  fn get_child_at_index(&mut self, index: i32) {
    unsafe {
      clutter_actor_get_child_at_index(self.as_actor(), index);
    }
  }

  /// Retrieves the number of children of the actor.
  ///
  /// _Since 1.10_
  fn get_n_children(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_actor_get_n_children(self.as_actor());
      return foreign_result;
    }
  }

  /// Retrieves the parent of the actor.
  ///
  /// _Since 1.10_
  fn get_parent(&mut self) -> ActorRef {
    unsafe {
      let foreign_result = clutter_actor_get_parent(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets `child` to be above `sibling` in the list of children of the actor.
  ///
  /// This method is logically equivalent to removing `child` and using
  /// `.insert_child_above()`, but it will not emit signals or change state
  /// on `child`.
  ///
  /// _Since 1.10_
  fn set_child_above_sibling<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
    unsafe {
      clutter_actor_set_child_above_sibling(self.as_actor(), child.as_actor(), sibling.as_actor());
    }
  }

  /// Changes the index of `child` in the list of children of the actor.
  ///
  /// This method is logically equivalent to removing `child` and calling
  /// `.insert_child_at_index()`, but it will not emit signals or change state
  /// on `child`.
  ///
  /// _Since 1.10_
  fn set_child_at_index<T: Actor>(&mut self, child: &mut T, index: i32) {
    unsafe {
      clutter_actor_set_child_at_index(self.as_actor(), child.as_actor(), index);
    }
  }

  /// Sets `child` to be below `sibling` in the list of children of the actor.
  ///
  /// This method is logically equivalent to removing the actor and using
  /// `.insert_child_below()`, but it will not emit signals or change state
  /// on `child`.
  ///
  /// _Since 1.10_
  fn set_child_below_sibling<T: Actor, U: Actor>(&mut self, child: &mut T, sibling: &mut U) {
    unsafe {
      clutter_actor_set_child_below_sibling(self.as_actor(), child.as_actor(), sibling.as_actor());
    }
  }

  /// Determines if descendant is contained inside the actor (either as an
  /// immediate child, or as a deeper descendant).
  ///
  /// If the actor and descendant point to the same actor then it will also
  /// return __true__.
  ///
  /// _Since 1.4_
  fn contains<T: Actor>(&mut self, descendant: &mut T) -> bool {
    unsafe {
      let foreign_result = clutter_actor_contains(self.as_actor(), descendant.as_actor());
      return foreign_result != 0;
    }
  }

  /// Retrieves the Stage where the actor is contained.
  ///
  /// _Since 0.8_
  fn get_stage(&mut self) -> ActorRef {
    unsafe {
      let foreign_result = clutter_actor_get_stage(self.as_actor());
      return foreign_result;
    }
  }

  /// Saves the current easing state for animatable properties, and creates a
  /// new state with the default values for easing mode and duration.
  ///
  /// New transitions created after calling this method will inherit the
  /// duration, easing mode, and delay of the new easing state; this also
  /// applies to transitions modified in flight.
  ///
  /// _Since 1.10_
  fn save_easing_state(&mut self) {
    unsafe {
      clutter_actor_save_easing_state(self.as_actor());
    }
  }

  /// Restores the easing state as it was prior to a call to
  /// `.save_easing_state()`.
  ///
  /// _Since 1.10_
  fn restore_easing_state(&mut self) {
    unsafe {
      clutter_actor_restore_easing_state(self.as_actor());
    }
  }

  /// Sets the duration of the tweening for animatable properties of the actor
  /// for the current easing state.
  ///
  /// _Since 1.10_
  fn set_easing_duration(&mut self, msecs: i32) {
    unsafe {
      clutter_actor_set_easing_duration(self.as_actor(), msecs);
    }
  }

  /// Retrieves the duration of the tweening for animatable properties of the
  /// actor for the current easing state.
  ///
  /// _Since 1.10_
  fn get_easing_duration(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_actor_get_easing_duration(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets the delay that should be applied before tweening animatable
  /// properties.
  ///
  /// _Since 1.10_
  fn set_easing_delay(&mut self, msecs: i32) {
    unsafe {
      clutter_actor_set_easing_delay(self.as_actor(), msecs);
    }
  }

  /// Retrieves the delay that should be applied when tweening animatable
  /// properties.
  ///
  /// _Since 1.10_
  fn get_easing_delay(&mut self) -> i32 {
    unsafe {
      let foreign_result = clutter_actor_get_easing_delay(self.as_actor());
      return foreign_result;
    }
  }

  /// Sets the actor as reactive. Reactive actors will receive events.
  ///
  /// _Since 0.6_
  fn set_reactive(&mut self, reactive: bool) {
    unsafe {
      clutter_actor_set_reactive(self.as_actor(), (reactive as i32));
    }
  }

  /// Checks whether the actor is marked as reactive.
  ///
  /// _Since 0.6_
  fn get_reactive(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_get_reactive(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Checks whether the actor is the Actor that has key focus.
  ///
  /// _Since 1.4_
  fn has_key_focus(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_has_key_focus(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Sets the key focus of the Stage including the actor to this Actor.
  ///
  /// _Since 1.0_
  fn grab_key_focus(&mut self) {
    unsafe {
      clutter_actor_grab_key_focus(self.as_actor());
    }
  }

  /// Checks whether the actor contains the pointer of an InputDevice.
  ///
  /// _Since 1.2_
  fn has_pointer(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_has_pointer(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Returns whether the actor has any actions applied.
  ///
  /// _Since 1.10_
  fn has_actions(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_has_actions(self.as_actor());
      return foreign_result != 0;
    }
  }

  /// Adds `constraint` to the list of Constraints applied to the actor.
  ///
  /// The Actor will hold a reference on the `constraint` until either
  /// `.remove_constraint()` or `.clear_constraints()` is called.
  ///
  /// _Since 1.4_
  fn add_constraint<T: Constraint>(&mut self, constraint: &mut T) {
    unsafe {
      clutter_actor_add_constraint(self.as_actor(), constraint.as_constraint());
    }
  }

  //FIXME doc
  fn on_allocation_changed(&mut self, handler: &|&mut ActorRef, &Box, allocation::Flags|) -> u64 {
    unsafe {
      let null_void: *mut libc::c_void = std::ptr::null_mut();
      return rsi_connect_on_allocation_changed(self.as_actor(), "allocation_changed".to_c_str().unwrap() as *mut i8, handler_for_on_allocation_changed, std::mem::transmute::<&|&mut ActorRef, &Box, allocation::Flags|, *mut libc::c_void>(handler), null_void, 0);
    }
  }

  //FIXME doc
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

//FIXME doc
extern "C" fn handler_for_on_allocation_changed(actor: *mut libc::c_void, allocation_box: *mut Box, flags: allocation::Flags, handler: *mut libc::c_void) {
  unsafe {
    let mut actor_r = ActorRef { opaque: actor };
    let handler = std::mem::transmute::<*mut libc::c_void, &mut |actor: &mut ActorRef, allocation_box: &Box, flags: allocation::Flags|>(handler);
    (*handler)(&mut actor_r, std::mem::transmute(allocation_box), flags);
    std::mem::forget(actor_r);
  }
}

//FIXME doc
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
  fn clutter_actor_set_pivot_point(self_value: *mut libc::c_void, pivot_x: f32, pivot_y: f32);
  fn clutter_actor_get_pivot_point(self_value: *mut libc::c_void, pivot_x: *mut f32, pivot_y: *mut f32);
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
  fn clutter_actor_grab_key_focus(self_value: *mut libc::c_void);
  fn clutter_actor_has_pointer(self_value: *mut libc::c_void) -> i32;
  fn clutter_actor_has_actions(self_value: *mut libc::c_void) -> i32;
  fn clutter_actor_add_constraint(self_value: *mut libc::c_void, constraint: *mut libc::c_void);
}

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct ActorMetaRef {
  opaque: *mut libc::c_void
}

/// The base trait of actor modifiers.
///
/// ActorMeta is a trait providing a common API for modifiers of Actor
/// behaviour, appearance or layout.
///
/// An ActorMeta can only be owned by a single Actor at any time.
///
/// Every sub-class (FIXME: Rust equivalent?) of ActorMeta should check if the
/// `enabled` property is set to __true__ before applying any kind of
/// modification.
///
/// _Since 1.4_
pub trait ActorMeta {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
  fn as_actor_meta(&self) -> *mut libc::c_void;

  /// Sets the name of meta.
  ///
  /// The name can be used to identify the ActorMeta instance.
  ///
  /// _Since 1.4_
  fn set_name(&mut self, name: &str) {
    unsafe {
      use std::c_str::ToCStr;
      clutter_actor_meta_set_name(self.as_actor_meta(), name.to_c_str().unwrap() as *mut i8);
    }
  }

  /// Retrieves the name set using `.set_name()`.
  ///
  /// _Since 1.4_
  fn get_name(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_actor_meta_get_name(self.as_actor_meta());
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  /// Sets whether meta should be enabled or not.
  ///
  /// _Since 1.4_
  fn set_enabled(&mut self, enabled: bool) {
    unsafe {
      clutter_actor_meta_set_enabled(self.as_actor_meta(), (enabled as i32));
    }
  }

  /// Retrieves whether meta is enabled.
  ///
  /// _Since 1.4_
  fn get_enabled(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_actor_meta_get_enabled(self.as_actor_meta());
      return foreign_result != 0;
    }
  }

  /// Retrieves a pointer to the Actor that owns meta.
  ///
  /// _Since 1.4_
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
  /// Destroys an actor.
  ///
  /// When an actor is destroyed, it will break any references it holds to other
  /// objects. If the actor is inside a container, the actor will be removed.
  /// When you destroy a container, its children will be destroyed as well.
  ///
  /// Note you cannot destroy the Stage returned by `Stage#get_default()`.
  fn drop(&mut self) {
    unsafe {
      clutter_actor_destroy(self.opaque);
    }
  }
}

extern {
  fn clutter_actor_destroy(self_value: *mut libc::c_void);
}
