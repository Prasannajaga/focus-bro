use tauri::Manager;
use tauri::{AppHandle, Runtime, plugin::TauriPlugin};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::fs::{self, File, create_dir_all};
use std::time::Duration;  

const DIR_NAME : &str = "focus-bro";
const FILE_NAME : &str = "focus.bro"; 
 
pub fn TARGET_KEYS() -> Shortcut {
    let target = Shortcut::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::Space,
    ); 

    target
}

pub fn register_shortcut<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, shortcut, event| {

 
            if shortcut == &TARGET_KEYS() {
                match event.state() {
                    ShortcutState::Pressed => {
                        let text = read_clipboard_variant_a(app);
                        write_file(&app,&text);
                        println!("Clipboard: {}", text);
                    }
                    ShortcutState::Released => {}
                }
            }
        })
        .build()
}
 

#[tauri::command]
pub async fn read_file(app : AppHandle) -> Result<Vec<String>, String> {
    let path: PathBuf = app
    .path()
    .app_local_data_dir()
    .expect("failed to load applocalData") 
    .join(DIR_NAME)
    .join(FILE_NAME);

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?; 
    
    let data: Vec<String> =
        serde_json::from_str(&content).map_err(|e: serde_json::Error| e.to_string())?;

    Ok(data)
}

pub fn write_file<R: Runtime>(app: &AppHandle<R>, text: &str) {
    let base_path = app
        .path()
        .app_local_data_dir()
        .expect("Failed to get app local data dir")
        .join(DIR_NAME);

    // Make sure folder exists
    if let Err(e) = create_dir_all(&base_path) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    let file_path = base_path.join(FILE_NAME);

    // Read existing JSON array or start new
    let mut arr: Vec<String> = if file_path.exists() {
        match fs::read_to_string(&file_path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => vec![],
        }
    } else { 
        vec![]
    };
 
    arr.push(text.to_string());

    // Serialize JSON
    let json_output = match serde_json::to_string_pretty(&arr) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to serialize JSON: {}", e);
            return;
        }
    }; 

    if let Err(e) = File::create(&file_path).and_then(|mut file| file.write_all(json_output.as_bytes())) {
        eprintln!("Failed to write file: {}", e);
    }
}

fn read_clipboard_variant_a<R: Runtime>(app: &AppHandle<R>) -> String {
    thread::sleep(Duration::from_millis(20));

    match app.clipboard().read_text() {
        Ok(text) => text, 
        Err(e) => {
            eprintln!("clipboard error: {}", e);  
            String::new()
        }
    }
}