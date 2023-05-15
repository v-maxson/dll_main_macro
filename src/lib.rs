pub use winapi::{
    shared::minwindef::{BOOL, TRUE, HINSTANCE, LPVOID, DWORD}, 
    um::{
        winnt::DLL_PROCESS_ATTACH, 
        libloaderapi::{
            DisableThreadLibraryCalls, 
            FreeLibraryAndExitThread
        }, 
        processthreadsapi::CreateThread, 
        handleapi::CloseHandle
    }
};

#[macro_export]
macro_rules! dll_main {
    ($func:expr) => {
        unsafe extern "system" fn wrapper(lp_param: LPVOID) -> DWORD {
            {
                ($func)()
            }

            FreeLibraryAndExitThread(lp_param as _, 0);
            unreachable!()
        }
        
        #[no_mangle]
        unsafe extern "stdcall" fn DllMain(
            hinst_dll: HINSTANCE,
            fdw_reason: DWORD,
            _lpv_reserved: LPVOID
        ) -> BOOL {
            if fdw_reason == DLL_PROCESS_ATTACH {
                DisableThreadLibraryCalls(hinst_dll);
        
                // Assume STD is not enabled.
                pub fn null_mut<T>() -> *mut T {
                    0x0 as *mut T
                }
        
                let handle = CreateThread(
                    null_mut(),
                    0,
                    Some(wrapper),
                    hinst_dll as _,
                    0,
                    null_mut()
                );
        
                if !handle.is_null() {
                    CloseHandle(handle);
                }
            }
        
            TRUE
        }
    };
}