use std::os::raw::c_char;
use std::ffi::{CStr, c_void};

use ffi_helpers::null_pointer_check;
use store::{create_view_model, ViewModel};

use crate::store::StateChange;

mod player;
mod store;

#[no_mangle]
pub extern "C" fn spot_init_view_model() -> *mut ViewModel {
    Box::into_raw(Box::new(create_view_model()))
}

#[no_mangle]
pub unsafe extern "C" fn spot_login(
    view_model: *mut ViewModel,
    user: *const c_char,
    pass: *const c_char,
) {
    null_pointer_check!(view_model);

    let user = CStr::from_ptr(user).to_str().unwrap_unchecked();
    let pass = CStr::from_ptr(pass).to_str().unwrap_unchecked();

    (*view_model).create_player(user, pass)
}

#[no_mangle]
pub unsafe extern "C" fn spot_play(
    view_model: *mut ViewModel,
    track_id: *const c_char,
) {
    null_pointer_check!(view_model);

    let track_id = CStr::from_ptr(track_id).to_str().unwrap_unchecked();

    (*view_model).play_track_sync(track_id.to_string());
}

#[no_mangle]
pub unsafe extern "C" fn spot_resume(
    view_model: *mut ViewModel,
) {
    null_pointer_check!(view_model);

    (*view_model).resume_sync();
}

#[no_mangle]
pub unsafe extern "C" fn spot_pause(
    view_model: *mut ViewModel,
) {
    null_pointer_check!(view_model);

    (*view_model).pause_sync();
}

#[no_mangle]
pub unsafe extern "C" fn spot_listen_for_events(
    view_model: *mut ViewModel,
    context: *mut c_void,
    is_track_loaded_cb: Option<unsafe extern "C" fn(bool, *mut c_void)>,
    is_playing_cb: Option<unsafe extern "C" fn(bool, *mut c_void)>,
) {
    null_pointer_check!(view_model);

    let context = context as usize;
    let mut rx = (*view_model).runtime.block_on((*view_model).listen());
    println!("got a receiver for is_track_loaded");

    (*view_model).runtime.spawn(async move {
        println!("spawned a thread for receiving state_changes");
        while let Some(changes) = rx.recv().await {
            for change in changes {
                match change {
                    StateChange::IsTrackLoaded(is_track_loaded) => {
                        if let Some(cb) = is_track_loaded_cb {
                            println!("is track loaded changed! {}", is_track_loaded);
                            cb(is_track_loaded, context as *mut c_void);
                        }
                    },
                    StateChange::IsPlaying(is_playing) => {
                        if let Some(cb) = is_playing_cb {
                            println!("is playing changed! {}", is_playing);
                            cb(is_playing, context as *mut c_void);
                        }
                    },
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        let view_model = spot_init_view_model();
        let user = CString::new(env::var("SPOT_USER").unwrap()).unwrap();
        let pass = CString::new(env::var("SPOT_PASS").unwrap()).unwrap();

        unsafe { spot_login(view_model, user.as_ptr(), pass.as_ptr()) };

        let track = CString::new("5xcunlfaZvD9BDQsLONI7A").unwrap();

        unsafe { spot_play(view_model, track.as_ptr()) };
    }
}
