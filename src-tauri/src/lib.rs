mod project;
mod config;
use std::path::Path;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_projects() -> Vec<String> {
    let dir = Path::new("C:/Users/hytalo/OneDrive/Desktop/trabalho");
    return project::get_child_dirs(&dir);
}
#[tauri::command]
fn open_project(project_path: String) {
    let dir = Path::new(&project_path);
    project::open_project(dir);
}
#[tauri::command]
fn save_root_folder(project_path: String) {
    
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, open_project, get_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
