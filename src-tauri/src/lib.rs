mod project;
use std::path::Path;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_projects() -> Vec<String>{
    let dir = Path::new("C:/Users/hytalo/OneDrive/Desktop/trabalho");
    return project::get_child_dirs(&dir);   
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,get_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
