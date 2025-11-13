use tauri::{AppHandle, Runtime, plugin::TauriPlugin};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
use std::thread;
use std::time::Duration;



pub fn register_shortcut<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, shortcut, event| {
            let target = Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::SHIFT),
                Code::Space,
            );

            if shortcut == &target {
                match event.state() {
                    ShortcutState::Pressed => {
                        let text = read_clipboard_variant_a(app);
                        println!("Clipboard: {}", text);
                    }
                    ShortcutState::Released => {}
                }
            }
        })
        .build()
}

fn read_clipboard_variant_a<R: Runtime>(app: &AppHandle<R>) -> String {
    thread::sleep(Duration::from_millis(20));

    match app.clipboard().read_text() {
        Ok(text) => text,// empty clipboard
        Err(e) => {
            eprintln!("clipboard error: {}", e);  // log error
            String::new()
        }
    }
}