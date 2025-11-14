// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/dev/dri").exists() {
            // SAFETY: There's potential for race conditions in a multi-threaded context.
            unsafe {
                std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
            }
        }
    }

    apothekensoft_lib::run()
}
