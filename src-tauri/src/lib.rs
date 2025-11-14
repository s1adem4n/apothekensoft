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
            if let Err(err) = rdev::listen(move |event| {
                if let Some(payload) = map_key_event(&event) {
                    if let Err(emit_err) = app.emit("global-key-event", payload) {
                        eprintln!("failed to emit global key event: {emit_err}");
                    }
                }
            }) {
                eprintln!("global key listener exited with error: {:?}", err);
            }
        })
        .expect("failed to spawn global key listener thread");
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
