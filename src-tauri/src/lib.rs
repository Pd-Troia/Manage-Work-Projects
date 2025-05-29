mod project;
mod config;
use std::path::Path;
use config::{Config, Settings};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_config() -> Settings{
    Config::new().get_settings()
}
#[tauri::command]
fn save_default_login(state: bool){
    let mut config = Config::new();
    print!("bool {}",state);
    print!("bool {:?}",&config);    
    config.save_default_login(state);
}
#[tauri::command]
fn get_projects() -> Vec<String> {
    let settings = Config::new();
    let root_path = settings.get_settings().root_folder;
    let dir = Path::new(&root_path);    
    return project::get_child_dirs(&dir);
}
#[tauri::command]
fn open_project(project_path: String) {    
    let dir = Path::new(&project_path);
    project::open_project(dir);    
}
#[tauri::command]
fn save_root_folder(root_path: String) {    
    Config::new().save_root_folder(root_path);
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir { file_name: Some("logs".to_string()) }
                ))
                .build()
        )
        .invoke_handler(tauri::generate_handler![greet,get_config,save_default_login, open_project, get_projects,save_root_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
