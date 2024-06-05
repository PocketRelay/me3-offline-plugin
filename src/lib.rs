#![warn(unused_crate_dependencies)]

use retour::GenericDetour;
use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows_sys::Win32::{
    Foundation::{BOOL, TRUE},
    Networking::WinInet::{
        InternetGetConnectedState, INTERNET_CONNECTION, INTERNET_CONNECTION_LAN,
    },
};

/// Windows DLL entrypoint for the plugin
#[no_mangle]
extern "stdcall" fn DllMain(_hmodule: isize, reason: u32, _: *mut ()) -> bool {
    if let DLL_PROCESS_ATTACH = reason {
        hook_internet_connected()
    }

    true
}

type InternetGetConnectedStateFn = unsafe extern "system" fn(*mut INTERNET_CONNECTION, u32) -> i32;

/// Applies a hook that tricks the program into thinking it always has
/// an active internet connection. (Allows playing offline)
pub fn hook_internet_connected() {
    unsafe {
        GenericDetour::new(
            InternetGetConnectedState as InternetGetConnectedStateFn,
            fake_internet_get_state,
        )
        .expect("Failed to create detour")
        .enable()
        .expect("Failed to enable detour")
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
