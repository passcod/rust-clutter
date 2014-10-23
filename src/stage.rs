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

