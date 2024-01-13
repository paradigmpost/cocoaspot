use std::os::raw::c_char;
use std::ffi::CStr;
use std::ptr;

use tokio::runtime::Runtime;

mod player;

#[no_mangle]
pub extern "C" fn spot_init_runtime() -> *mut Runtime {
    Box::into_raw(Box::new(Runtime::new().unwrap()))
}

#[no_mangle]
pub unsafe extern "C" fn spot_init_player(
    runtime: *mut Runtime,
    user: *const c_char,
    pass: *const c_char,
) -> *mut player::Instance {
    let user = CStr::from_ptr(user).to_str().unwrap_unchecked();
    let pass = CStr::from_ptr(pass).to_str().unwrap_unchecked();

    return match player::create(runtime, user, pass) {
        None => ptr::null_mut(),
        Some(instance) => Box::into_raw(Box::new(instance)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn spot_play(
    runtime: *mut Runtime,
    instance: *mut player::Instance,
    track_id: *const c_char,
) {
    let track_id = CStr::from_ptr(track_id).to_str().unwrap_unchecked();

    player::play_sync(runtime, instance, track_id);
}

#[no_mangle]
pub unsafe extern "C" fn spot_resume(
    instance: *mut player::Instance,
) {
    player::resume(instance)
}

#[no_mangle]
pub unsafe extern "C" fn spot_pause(
    instance: *mut player::Instance,
) {
    player::pause(instance);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        let runtime = spot_init_runtime();
        let user = 
            CString::new("samoore11@gmail.com").unwrap();
        let pass =
            CString::new(env::var("SPOT_PASS").unwrap()).unwrap();
        let player = unsafe { spot_init_player(runtime, user.as_ptr(), pass.as_ptr()) };
        let track =
            CString::new("5xcunlfaZvD9BDQsLONI7A").unwrap();

        unsafe { spot_play(
            runtime,
            player,
            track.as_ptr(),
        ) };
    }
}
