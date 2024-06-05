#![warn(unused_crate_dependencies)]

use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

pub mod hooks;

/// Handles the plugin being attached to the game
fn attach() {
    // Debug allocates a console window to display output
    #[cfg(debug_assertions)]
    {
        unsafe { windows_sys::Win32::System::Console::AllocConsole() };
    }

    // Apply the host lookup hook
    unsafe { hooks::hook_internet_connected() };
}

/// Handles the plugin being detached from the game, this handles
/// cleaning up any extra allocated resources
fn detach() {
    // Debug console must be freed on detach
    #[cfg(debug_assertions)]
    {
        unsafe {
            windows_sys::Win32::System::Console::FreeConsole();
        }
    }
}

/// Windows DLL entrypoint for the plugin
#[no_mangle]
extern "stdcall" fn DllMain(_hmodule: isize, reason: u32, _: *mut ()) -> bool {
    match reason {
        // Handle attaching
        DLL_PROCESS_ATTACH => attach(),
        // Handle detaching
        DLL_PROCESS_DETACH => detach(),
        _ => {}
    }

    true
}
