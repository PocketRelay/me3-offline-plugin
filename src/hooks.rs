use retour::GenericDetour;
use windows_sys::Win32::{
    Foundation::{BOOL, TRUE},
    Networking::WinInet::{
        InternetGetConnectedState, INTERNET_CONNECTION, INTERNET_CONNECTION_LAN,
    },
};

type InternetGetConnectedStateFn = unsafe extern "system" fn(*mut INTERNET_CONNECTION, u32) -> i32;

/// Applies a hook that tricks the program into thinking it always has
/// an active internet connection. (Allows playing offline)
///
/// ## Safety
///
/// This function replaces a memory address redirecting to a matching function.
pub unsafe fn hook_internet_connected() {
    unsafe {
        GenericDetour::new(
            InternetGetConnectedState as InternetGetConnectedStateFn,
            fake_internet_get_state,
        )
        .unwrap()
        .enable()
        .unwrap()
    };
}

/// Offline check that always returns TRUE
///
/// ## Safety
///
/// Doesn't perform any unsafe actions, just must be marked as unsafe to
/// be used as an extern fn
#[no_mangle]
pub unsafe extern "system" fn fake_internet_get_state(
    lpdw_flags: *mut INTERNET_CONNECTION,
    _dw_reserved: u32,
) -> BOOL {
    if !lpdw_flags.is_null() {
        *lpdw_flags = INTERNET_CONNECTION_LAN;
    }
    TRUE
}
