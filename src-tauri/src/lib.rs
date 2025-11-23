mod tools;

use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_clipboard_manager;
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tools::{read_file, register_shortcut, TARGET_KEYS};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(register_shortcut())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let _ = app.autolaunch().enable();

            app.global_shortcut().register(TARGET_KEYS())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_file, open_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_url(url: String) {
    std::process::Command::new("cmd")
        .args(["/c", "start", &url])
        .spawn()
        .unwrap();
}
