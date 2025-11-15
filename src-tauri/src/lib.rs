use rdev::{Event, EventType, Key};
use serde::Serialize;
use std::{thread, time};
use tauri::{AppHandle, Emitter};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum KeyState {
    Down,
    Up,
}

#[derive(Clone, Debug, Serialize)]
struct GlobalKeyEventPayload {
    key_code: String,
    label: Option<String>,
    state: KeyState,
}

#[tauri::command]
fn simulate_key(key: String) -> Result<(), String> {
    let parsed_key = parse_key(&key).ok_or_else(|| format!("Invalid key: {}", key))?;

    rdev::simulate(&EventType::KeyPress(parsed_key))
        .map_err(|e| format!("Failed to simulate key: {:?}", e))?;
    thread::sleep(time::Duration::from_millis(100));
    rdev::simulate(&EventType::KeyRelease(parsed_key))
        .map_err(|e| format!("Failed to simulate key: {:?}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![simulate_key])
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle().clone();
            spawn_global_key_listener(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn spawn_global_key_listener(app: AppHandle) {
    std::thread::Builder::new()
        .name("global-key-listener".into())
        .spawn(move || {
            #[cfg(windows)]
            {
                windows_key_listener(app);
            }
            #[cfg(not(windows))]
            {
                if let Err(err) = rdev::listen(move |event| {
                    if let Some(payload) = map_key_event(&event) {
                        if let Err(emit_err) = app.emit("global-key-event", payload) {
                            eprintln!("failed to emit global key event: {emit_err}");
                        }
                    }
                }) {
                    eprintln!("global key listener exited with error: {:?}", err);
                }
            }
        })
        .expect("failed to spawn global key listener thread");
}

#[cfg(windows)]
fn windows_key_listener(app: AppHandle) {
    use windows::Win32::Foundation::LPARAM;
    use windows::Win32::Foundation::LRESULT;
    use windows::Win32::Foundation::WPARAM;
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    use windows::Win32::UI::WindowsAndMessaging::*;

    unsafe extern "system" fn keyboard_hook_proc(
        code: i32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if code >= 0 {
            let kb_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let vk_code = kb_struct.vkCode;

            // Get the app handle from thread-local storage
            if let Some(app) = APP_HANDLE.with(|h| h.borrow().clone()) {
                let is_key_down =
                    wparam.0 == WM_KEYDOWN as usize || wparam.0 == WM_SYSKEYDOWN as usize;
                let is_key_up = wparam.0 == WM_KEYUP as usize || wparam.0 == WM_SYSKEYUP as usize;

                if is_key_down || is_key_up {
                    if let Some(key_info) = virtual_key_to_info(vk_code) {
                        let payload = GlobalKeyEventPayload {
                            key_code: key_info.0.to_string(),
                            label: Some(key_info.1.to_string()),
                            state: if is_key_down {
                                KeyState::Down
                            } else {
                                KeyState::Up
                            },
                        };

                        if let Err(emit_err) = app.emit("global-key-event", payload) {
                            eprintln!("failed to emit global key event: {emit_err}");
                        }
                    }
                }
            }
        }

        // CRITICAL: Always call CallNextHookEx to pass the event to other applications
        unsafe { CallNextHookEx(None, code, wparam, lparam) }
    }

    use std::cell::RefCell;
    thread_local! {
        static APP_HANDLE: RefCell<Option<AppHandle>> = RefCell::new(None);
    }

    APP_HANDLE.with(|h| *h.borrow_mut() = Some(app));

    unsafe {
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook_proc), None, 0);

        if hook.is_err() {
            eprintln!("Failed to set Windows keyboard hook");
            return;
        }

        // Message loop to keep the hook alive
        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = UnhookWindowsHookEx(hook.unwrap());
    }
}

#[cfg(windows)]
fn virtual_key_to_info(vk: u32) -> Option<(&'static str, &'static str)> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    match VIRTUAL_KEY(vk as u16) {
        // Letters
        VIRTUAL_KEY(0x41) => Some(("KeyA", "a")),
        VIRTUAL_KEY(0x42) => Some(("KeyB", "b")),
        VIRTUAL_KEY(0x43) => Some(("KeyC", "c")),
        VIRTUAL_KEY(0x44) => Some(("KeyD", "d")),
        VIRTUAL_KEY(0x45) => Some(("KeyE", "e")),
        VIRTUAL_KEY(0x46) => Some(("KeyF", "f")),
        VIRTUAL_KEY(0x47) => Some(("KeyG", "g")),
        VIRTUAL_KEY(0x48) => Some(("KeyH", "h")),
        VIRTUAL_KEY(0x49) => Some(("KeyI", "i")),
        VIRTUAL_KEY(0x4A) => Some(("KeyJ", "j")),
        VIRTUAL_KEY(0x4B) => Some(("KeyK", "k")),
        VIRTUAL_KEY(0x4C) => Some(("KeyL", "l")),
        VIRTUAL_KEY(0x4D) => Some(("KeyM", "m")),
        VIRTUAL_KEY(0x4E) => Some(("KeyN", "n")),
        VIRTUAL_KEY(0x4F) => Some(("KeyO", "o")),
        VIRTUAL_KEY(0x50) => Some(("KeyP", "p")),
        VIRTUAL_KEY(0x51) => Some(("KeyQ", "q")),
        VIRTUAL_KEY(0x52) => Some(("KeyR", "r")),
        VIRTUAL_KEY(0x53) => Some(("KeyS", "s")),
        VIRTUAL_KEY(0x54) => Some(("KeyT", "t")),
        VIRTUAL_KEY(0x55) => Some(("KeyU", "u")),
        VIRTUAL_KEY(0x56) => Some(("KeyV", "v")),
        VIRTUAL_KEY(0x57) => Some(("KeyW", "w")),
        VIRTUAL_KEY(0x58) => Some(("KeyX", "x")),
        VIRTUAL_KEY(0x59) => Some(("KeyY", "y")),
        VIRTUAL_KEY(0x5A) => Some(("KeyZ", "z")),
        // Numbers
        VIRTUAL_KEY(0x30) => Some(("Num0", "0")),
        VIRTUAL_KEY(0x31) => Some(("Num1", "1")),
        VIRTUAL_KEY(0x32) => Some(("Num2", "2")),
        VIRTUAL_KEY(0x33) => Some(("Num3", "3")),
        VIRTUAL_KEY(0x34) => Some(("Num4", "4")),
        VIRTUAL_KEY(0x35) => Some(("Num5", "5")),
        VIRTUAL_KEY(0x36) => Some(("Num6", "6")),
        VIRTUAL_KEY(0x37) => Some(("Num7", "7")),
        VIRTUAL_KEY(0x38) => Some(("Num8", "8")),
        VIRTUAL_KEY(0x39) => Some(("Num9", "9")),
        // Special keys
        VK_RETURN => Some(("Return", "Enter")),
        VK_ESCAPE => Some(("Escape", "Esc")),
        VK_BACK => Some(("Backspace", "Backspace")),
        VK_TAB => Some(("Tab", "Tab")),
        VK_SPACE => Some(("Space", " ")),
        VK_LCONTROL => Some(("ControlLeft", "Ctrl")),
        VK_RCONTROL => Some(("ControlRight", "RCtrl")),
        VK_LSHIFT => Some(("ShiftLeft", "Shift")),
        VK_RSHIFT => Some(("ShiftRight", "RShift")),
        VK_LMENU => Some(("AltLeft", "Alt")),
        VK_RMENU => Some(("AltRight", "RAlt")),
        VK_LWIN => Some(("MetaLeft", "Win")),
        VK_RWIN => Some(("MetaRight", "RWin")),
        // Arrow keys
        VK_UP => Some(("UpArrow", "Up")),
        VK_DOWN => Some(("DownArrow", "Down")),
        VK_LEFT => Some(("LeftArrow", "Left")),
        VK_RIGHT => Some(("RightArrow", "Right")),
        // Function keys
        VK_F1 => Some(("F1", "F1")),
        VK_F2 => Some(("F2", "F2")),
        VK_F3 => Some(("F3", "F3")),
        VK_F4 => Some(("F4", "F4")),
        VK_F5 => Some(("F5", "F5")),
        VK_F6 => Some(("F6", "F6")),
        VK_F7 => Some(("F7", "F7")),
        VK_F8 => Some(("F8", "F8")),
        VK_F9 => Some(("F9", "F9")),
        VK_F10 => Some(("F10", "F10")),
        VK_F11 => Some(("F11", "F11")),
        VK_F12 => Some(("F12", "F12")),
        _ => None,
    }
}

fn map_key_event(event: &Event) -> Option<GlobalKeyEventPayload> {
    match event.event_type {
        EventType::KeyPress(key) => Some(GlobalKeyEventPayload {
            key_code: format!("{:?}", key),
            label: event.name.clone(),
            state: KeyState::Down,
        }),
        EventType::KeyRelease(key) => Some(GlobalKeyEventPayload {
            key_code: format!("{:?}", key),
            label: event.name.clone(),
            state: KeyState::Up,
        }),
        _ => None,
    }
}

fn parse_key(key_str: &str) -> Option<Key> {
    match key_str {
        // Letters
        "a" | "A" => Some(Key::KeyA),
        "b" | "B" => Some(Key::KeyB),
        "c" | "C" => Some(Key::KeyC),
        "d" | "D" => Some(Key::KeyD),
        "e" | "E" => Some(Key::KeyE),
        "f" | "F" => Some(Key::KeyF),
        "g" | "G" => Some(Key::KeyG),
        "h" | "H" => Some(Key::KeyH),
        "i" | "I" => Some(Key::KeyI),
        "j" | "J" => Some(Key::KeyJ),
        "k" | "K" => Some(Key::KeyK),
        "l" | "L" => Some(Key::KeyL),
        "m" | "M" => Some(Key::KeyM),
        "n" | "N" => Some(Key::KeyN),
        "o" | "O" => Some(Key::KeyO),
        "p" | "P" => Some(Key::KeyP),
        "q" | "Q" => Some(Key::KeyQ),
        "r" | "R" => Some(Key::KeyR),
        "s" | "S" => Some(Key::KeyS),
        "t" | "T" => Some(Key::KeyT),
        "u" | "U" => Some(Key::KeyU),
        "v" | "V" => Some(Key::KeyV),
        "w" | "W" => Some(Key::KeyW),
        "x" | "X" => Some(Key::KeyX),
        "y" | "Y" => Some(Key::KeyY),
        "z" | "Z" => Some(Key::KeyZ),
        // Numbers
        "0" => Some(Key::Num0),
        "1" => Some(Key::Num1),
        "2" => Some(Key::Num2),
        "3" => Some(Key::Num3),
        "4" => Some(Key::Num4),
        "5" => Some(Key::Num5),
        "6" => Some(Key::Num6),
        "7" => Some(Key::Num7),
        "8" => Some(Key::Num8),
        "9" => Some(Key::Num9),
        // Special keys
        "Return" | "Enter" => Some(Key::Return),
        "Escape" | "Esc" => Some(Key::Escape),
        "Backspace" => Some(Key::Backspace),
        "Tab" => Some(Key::Tab),
        "Space" => Some(Key::Space),
        "ControlLeft" | "Ctrl" => Some(Key::ControlLeft),
        "ShiftLeft" | "Shift" => Some(Key::ShiftLeft),
        "AltLeft" | "Alt" => Some(Key::Alt),
        "MetaLeft" | "Meta" | "Super" => Some(Key::MetaLeft),
        "ControlRight" => Some(Key::ControlRight),
        "ShiftRight" => Some(Key::ShiftRight),
        "AltRight" => Some(Key::AltGr),
        "MetaRight" => Some(Key::MetaRight),
        // Arrow keys
        "UpArrow" | "Up" => Some(Key::UpArrow),
        "DownArrow" | "Down" => Some(Key::DownArrow),
        "LeftArrow" | "Left" => Some(Key::LeftArrow),
        "RightArrow" | "Right" => Some(Key::RightArrow),
        // Function keys
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        _ => None,
    }
}
