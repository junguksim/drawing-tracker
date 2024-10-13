#[tauri::command]
fn get_focused_app_name() -> String {
    #[cfg(target_os = "macos")]
    {
        // macOS 구현 코드
        use objc::runtime::{Object, Class};
        use objc::{msg_send, sel, sel_impl};
        use std::ffi::CStr;
        use libc::c_char;

        unsafe {
            let workspace: *mut Object =
                msg_send![Class::get("NSWorkspace").unwrap(), sharedWorkspace];
            if workspace.is_null() {
                return String::from("Unknown");
            }

            let active_app: *mut Object = msg_send![workspace, frontmostApplication];
            if active_app.is_null() {
                return String::from("Unknown");
            }

            let app_name_nsstring: *mut Object = msg_send![active_app, localizedName];
            if app_name_nsstring.is_null() {
                return String::from("Unknown");
            }

            let cstr: *const c_char = msg_send![app_name_nsstring, UTF8String];
            if cstr.is_null() {
                return String::from("Unknown");
            }

            let app_name = CStr::from_ptr(cstr).to_string_lossy().into_owned();
            app_name
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows 구현 코드
        use windows::{
            core::*, // 필요한 경우 추가
            Win32::{
                Foundation::{MAX_PATH, HANDLE},
                System::{
                    ProcessStatus::K32GetModuleFileNameExW,
                    Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ},
                },
                UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
            },
        };
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use std::path::Path;

        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0 == 0 {
                return String::from("No window");
            }

            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);

            if pid == 0 {
                return String::from("Unknown");
            }

            let h_process = OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ,
                false,
                pid,
            );

            if h_process.is_invalid() {
                return String::from("Access denied");
            }

            let mut filename = [0u16; MAX_PATH as usize];
            let len = K32GetModuleFileNameExW(
                h_process,
                HANDLE(0),
                &mut filename,
            );
            if len == 0 {
                return String::from("Unknown");
            }

            let path = OsString::from_wide(&filename[..len as usize]);
            let app_name = Path::new(&path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string();

            app_name
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        String::from("Not implemented on this platform")
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_focused_app_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
