mod tools;
use tools::{register_shortcut };
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use tauri_plugin_clipboard_manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(
      register_shortcut()
    )
    .plugin(tauri_plugin_clipboard_manager::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      } 

      // Register the shortcut
      let shortcut = Shortcut::new(
          Some(Modifiers::CONTROL | Modifiers::SHIFT),
          Code::Space,
      );

      app.global_shortcut().register(shortcut)?;  

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

