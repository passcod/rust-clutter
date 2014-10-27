#![stable]

use libc;
use std;
use super::actor::Actor;

/// Opaque struct which holds a reference to the underlying Clutter object.
#[repr(C)]
pub struct StageRef {
  opaque: *mut libc::c_void
}

impl StageRef {
  /// Creates a new, non-default stage.
  ///
  /// A non-default stage is a new top-level actor which can be used as another
  /// container. It works exactly like the default stage, but while
  /// `.get_default()` will always return the same instance, you will have to
  /// keep a pointer to any Stage returned by `StageRef::new()`.
  ///
  /// The ability to support multiple stages depends on the current backend.
  /// Use `feature_available()` and CLUTTER_FEATURE_STAGE_MULTIPLE to check at
  /// runtime whether a backend supports multiple stages.
  ///
  /// _Since 0.8_
  pub fn new() -> StageRef {
    unsafe {
      let foreign_result = clutter_stage_new();
      return foreign_result;
    }
  }
}

/// Top level visual element to which actors are placed.
///
/// Stage is a top level 'window' on which child actors are placed and
/// manipulated.
///
/// Backends might provide support for multiple stages. The support for this
/// feature can be checked at runtime using the `feature_available()` function
/// and the CLUTTER_FEATURE_STAGE_MULTIPLE flag. If the backend used supports
/// multiple stages, new Stage instances can be created using
/// `StageRef::new()`.
///
/// Stage is a proxy actor, wrapping the backend-specific implementation of the
/// windowing system. It is possible to subclass (FIXME: Rust equivalent?)
/// Stage, as long as every overridden virtual function chains up to the parent
/// class corresponding function.
pub trait Stage {
  /// Returns a pointer to the the underlying C object.
  ///
  /// Generally only used internally.
  fn as_stage(&self) -> *mut libc::c_void;

  /// Makes sure the right GL context is current for the passed stage.
  ///
  /// Not intended to be used by applications.
  ///
  /// _Since 0.8_
  fn ensure_current(&mut self) {
    unsafe {
      clutter_stage_ensure_current(self.as_stage());
    }
  }

  /// Ensures that the GL viewport is updated with the current stage window size.
  ///
  /// This method will queue a redraw of the stage.
  ///
  /// It should not be called by applications; it is used when embedding a Stage
  /// into a toolkit with another windowing system, like GTK+.
  ///
  /// _Since 1.0_
  fn ensure_viewport(&mut self) {
    unsafe {
      clutter_stage_ensure_viewport(self.as_stage());
    }
  }

  /// Ensures that stage is redrawn.
  ///
  /// This method should not be called by applications: it is used when embedding
  /// a Stage into a toolkit with another windowing system, like GTK+.
  ///
  /// _Since 1.0_
  fn ensure_redraw(&mut self) {
    unsafe {
      clutter_stage_ensure_redraw(self.as_stage());
    }
  }

  /// Sets the key focus on `actor`.
  ///
  /// An actor with key focus will receive all the key events.
  ///
  /// _Since 0.6_
  fn set_key_focus<T: Actor>(&mut self, actor: &mut T) {
    unsafe {
      clutter_stage_set_key_focus(self.as_stage(), actor.as_actor());
    }
  }

  /// Retrieves the actor that is currently under key focus.
  ///
  /// _Since 0.6_
  fn get_key_focus(&mut self) -> super::actor::ActorRef {
    unsafe {
      let foreign_result = clutter_stage_get_key_focus(self.as_stage());
      return foreign_result;
    }
  }

  /// Sets whether motion events received between redraws should be throttled
  /// or not.
  ///
  /// If motion events are throttled, those events received by the windowing
  /// system between redraws will be compressed so that only the last event
  /// will be propagated to the stage and its actors.
  ///
  /// This method should only be used if you want to have all the motion events
  /// delivered to your application code.
  ///
  /// _Since 1.0_
  fn set_throttle_motion_events(&mut self, throttle: bool) {
    unsafe {
      clutter_stage_set_throttle_motion_events(self.as_stage(), (throttle as i32));
    }
  }

  /// Retrieves the value set with `.set_throttle_motion_events()`.
  ///
  /// _Since 1.0_
  fn get_throttle_motion_events(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_stage_get_throttle_motion_events(self.as_stage());
      return foreign_result != 0;
    }
  }

  /// Sets whether the stage should honour the `opacity` and the alpha channel
  /// of the `color`.
  ///
  /// _Since 1.2_
  fn set_use_alpha(&mut self, use_alpha: bool) {
    unsafe {
      clutter_stage_set_use_alpha(self.as_stage(), (use_alpha as i32));
    }
  }

  /// Retrieves the value set using `.set_use_alpha()`.
  ///
  /// _Since 1.2_
  fn get_use_alpha(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_stage_get_use_alpha(self.as_stage());
      return foreign_result != 0;
    }
  }

  /// Sets the minimum size for a stage window, if the default backend uses
  /// Stage inside a window.
  ///
  /// This is a convenience method, and it is equivalent to setting the
  /// `min-width` and `min-height` on the stage.
  ///
  /// If the current size of the stage is smaller than the minimum size, the
  /// stage will be resized to the new width and height.
  ///
  /// This method has no effect if the stage is fullscreen.
  ///
  /// _Since 1.2_
  fn set_minimum_size(&mut self, width: i32, height: i32) {
    unsafe {
      clutter_stage_set_minimum_size(self.as_stage(), width, height);
    }
  }

  /// Retrieves the minimum size for a stage window as set using
  /// `.set_minimum_size()`.
  ///
  /// The returned size may not correspond to the actual minimum size and it is
  /// specific to the Stage implementation inside the Clutter backend.
  ///
  /// _Since 1.2_
  fn get_minimum_size(&mut self) -> (i32, i32) {
    unsafe {
      let mut width:i32 = std::intrinsics::init();
      let mut height:i32 = std::intrinsics::init();
      clutter_stage_get_minimum_size(self.as_stage(), &mut width, &mut height);
      return (width, height);
    }
  }

  /// Sets whether the stage should clear itself at the beginning of each paint
  /// cycle or not.
  ///
  /// Clearing the Stage can be a costly operation, especially if the stage is
  /// always covered — for example, in a full-screen video player or in a game
  /// with a background texture.
  ///
  /// This setting is a hint; Clutter might discard this hint depending on its
  /// internal state.
  ///
  /// If parts of the stage are visible and you disable clearing you might end
  /// up with visual artifacts while painting the contents of the stage.
  ///
  /// _Since 1.4_
  fn set_no_clear_hint(&mut self, use_alpha: bool) {
    unsafe {
      clutter_stage_set_no_clear_hint(self.as_stage(), (use_alpha as i32));
    }
  }

  /// Retrieves the hint set with `.set_no_clear_hint()`.
  ///
  /// _Since 1.4_
  fn get_no_clear_hint(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_stage_get_no_clear_hint(self.as_stage());
      return foreign_result != 0;
    }
  }

  /// Sets whether per-actor motion events (and relative crossing events)
  /// should be disabled or not.
  ///
  /// The default is __true__.
  ///
  /// If enable is __false__ the following signals will not be emitted by the
  /// actors children of the stage:
  ///
  /// - `motion-event`
  /// - `enter-event`
  /// - `leave-event`
  ///
  /// The events will still be delivered to the Stage.
  ///
  /// The main side effect of this function is that disabling the motion events
  /// will disable picking to detect the Actor underneath the pointer for each
  /// motion event. This is useful, for instance, when dragging an Actor across
  /// the stage: the actor underneath the pointer is not going to change, so
  /// it's meaningless to perform a pick.
  ///
  /// _Since 1.8_
  fn set_motion_events_enabled(&mut self, enabled: bool) {
    unsafe {
      clutter_stage_set_motion_events_enabled(self.as_stage(), (enabled as i32));
    }
  }

  /// Retrieves the value set using `.set_motion_events_enabled()`.
  ///
  /// _Since 1.8_
  fn get_motion_events_enabled(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_stage_get_motion_events_enabled(self.as_stage());
      return foreign_result != 0;
    }
  }

  /// Sets the stage title.
  ///
  /// _Since 0.4_
  fn set_title(&mut self, title: &str) {
    unsafe {
      use std::c_str::ToCStr;
      clutter_stage_set_title(self.as_stage(), title.to_c_str().unwrap() as *mut i8);
    }
  }

  /// Gets the stage title.
  ///
  /// _Since 0.4_
  fn get_title(&mut self) -> std::c_str::CString {
    unsafe {
      let foreign_result = clutter_stage_get_title(self.as_stage());
      return std::c_str::CString::new(foreign_result as *const i8, false);
    }
  }

  /// Sets if the stage is resizable by user interaction (e.g. via window
  /// manager controls).
  ///
  /// _Since 0.4_
  fn set_user_resizable(&mut self, resizable: bool) {
    unsafe {
      clutter_stage_set_user_resizable(self.as_stage(), (resizable as i32));
    }
  }

  /// Retrieves the value set with `.set_user_resizable()`.
  ///
  /// _Since 0.4_
  fn get_user_resizable(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_stage_get_user_resizable(self.as_stage());
      return foreign_result != 0;
    }
  }

  /// Asks to place the stage window in the fullscreen or unfullscreen states.
  ///
  /// (Note that you shouldn't assume the window is definitely fullscreen
  /// afterward, because other entities (e.g. the user or window manager) could
  /// unfullscreen it again, and not all window managers honor requests to
  /// fullscreen windows.)
  ///
  /// If you want to receive notification of the fullscreen state you should
  /// either use the `fullscreen` and `unfullscreen` signals, or use the notify
  /// signal for the `fullscreen-set` property.
  ///
  /// _Since 1.0_
  fn set_fullscreen(&mut self, fullscreen: bool) {
    unsafe {
      clutter_stage_set_fullscreen(self.as_stage(), (fullscreen as i32));
    }
  }

  /// Retrieves whether the stage is fullscreen or not.
  ///
  /// _Since 1.0_
  fn get_fullscreen(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_stage_get_fullscreen(self.as_stage());
      return foreign_result != 0;
    }
  }

  /// Shows the cursor on the stage window.
  fn show_cursor(&mut self) {
    unsafe {
      clutter_stage_show_cursor(self.as_stage());
    }
  }

  /// Makes the cursor invisible on the stage window.
  ///
  /// _Since 0.4_
  fn hide_cursor(&mut self) {
    unsafe {
      clutter_stage_hide_cursor(self.as_stage());
    }
  }

  /// Sets whether the stage should accept the key focus when shown.
  ///
  /// This method should be called before showing the stage using
  /// `Actor#show()`.
  ///
  /// _Since 1.6_
  fn set_accept_focus(&mut self, accept_focus: bool) {
    unsafe {
      clutter_stage_set_accept_focus(self.as_stage(), (accept_focus as i32));
    }
  }

  /// Retrieves the value set with `.set_accept_focus()`.
  ///
  /// _Since 1.6_
  fn get_accept_focus(&mut self) -> bool {
    unsafe {
      let foreign_result = clutter_stage_get_accept_focus(self.as_stage());
      return foreign_result != 0;
    }
  }

  /// Enables an alternate behavior where Clutter draws at a fixed point in
  /// time after the frame presentation time (also known as the VBlank time).
  ///
  /// This is most useful when the application wants to show incoming data
  /// with predictable latency. (The primary example of this would be a
  /// window system compositor.) By synchronizing to provide new data before
  /// Clutter redraws, an external source of updates (in the compositor, an
  /// application) can get a reliable latency.
  ///
  /// The appropriate value of `sync_delay` depends on the complexity of
  /// drawing the stage's scene graph — in general a value of between 0 and
  /// 8 ms (up to one-half of a typical 60hz frame rate) is appropriate. Using
  /// a larger value will reduce latency but risks skipping a frame if drawing
  /// the stage takes too long.
  ///
  /// If `sync_delay` is less than zero, this method restores the default
  /// behavior where redraw is throttled to the refresh rate but not
  /// synchronized to it.
  ///
  /// _Since 1.14_
  #[unstable]
  fn set_sync_delay(&mut self, accept_focus: i32) {
    unsafe {
      clutter_stage_set_sync_delay(self.as_stage(), accept_focus);
    }
  }

  /// Causes the next frame for the stage to be drawn as quickly as possible,
  /// ignoring any delay that `.set_sync_delay()` would normally cause.
  ///
  /// _Since 1.14_
  #[unstable]
  fn skip_sync_delay(&mut self) {
    unsafe {
      clutter_stage_skip_sync_delay(self.as_stage());
    }
  }

  //FIXME: doc
  fn on_activate(&mut self, handler: &|&mut StageRef|) -> u64 {
    unsafe {
      let null_void: *mut libc::c_void = std::ptr::null_mut();
      return rsi_connect_on_activate(self.as_stage(), "activate".to_c_str().unwrap() as *mut i8, handler_for_on_activate, std::mem::transmute::<&|&mut StageRef|, *mut libc::c_void>(handler), null_void, 0);
    }
  }

  //FIXME: doc
  fn on_deactivate(&mut self, handler: &|&mut StageRef|) -> u64 {
    unsafe {
      let null_void: *mut libc::c_void = std::ptr::null_mut();
      return rsi_connect_on_deactivate(self.as_stage(), "deactivate".to_c_str().unwrap() as *mut i8, handler_for_on_deactivate, std::mem::transmute::<&|&mut StageRef|, *mut libc::c_void>(handler), null_void, 0);
    }
  }

  //FIXME: doc
  fn on_fullscreen(&mut self, handler: &|&mut StageRef|) -> u64 {
    unsafe {
      let null_void: *mut libc::c_void = std::ptr::null_mut();
      return rsi_connect_on_fullscreen(self.as_stage(), "fullscreen".to_c_str().unwrap() as *mut i8, handler_for_on_fullscreen, std::mem::transmute::<&|&mut StageRef|, *mut libc::c_void>(handler), null_void, 0);
    }
  }

  //FIXME: doc
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

//FIXME: doc
extern "C" fn handler_for_on_activate(stage: *mut libc::c_void, handler: *mut libc::c_void) {
  unsafe {
    let mut stage_r = StageRef { opaque: stage };
    let handler = std::mem::transmute::<*mut libc::c_void, &mut |stage: &mut StageRef|>(handler);
    (*handler)(&mut stage_r);
    std::mem::forget(stage_r);
  }
}

//FIXME: doc
extern "C" fn handler_for_on_deactivate(stage: *mut libc::c_void, handler: *mut libc::c_void) {
  unsafe {
    let mut stage_r = StageRef { opaque: stage };
    let handler = std::mem::transmute::<*mut libc::c_void, &mut |stage: &mut StageRef|>(handler);
    (*handler)(&mut stage_r);
    std::mem::forget(stage_r);
  }
}

//FIXME: doc
extern "C" fn handler_for_on_fullscreen(stage: *mut libc::c_void, handler: *mut libc::c_void) {
  unsafe {
    let mut stage_r = StageRef { opaque: stage };
    let handler = std::mem::transmute::<*mut libc::c_void, &mut |stage: &mut StageRef|>(handler);
    (*handler)(&mut stage_r);
    std::mem::forget(stage_r);
  }
}

//FIXME: doc
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
