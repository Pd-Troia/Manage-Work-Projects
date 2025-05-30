use std::{env};
use std::path::Path;
use std::process::Command;

use crate::Config;


fn open_vscode(path: &String) {
    if let Ok(_) = env::set_current_dir(path) {
        Command::new("cmd")
            .args(["/C", "code ."])
            .output()
            .expect("falha ao abrir vscode");
    }    
}
fn login_vendor(vendor: &str){    
    let settings = Config::new().get_settings();  
    print!("{}",&settings.default_login); 
    let command = format!("vtex {}",&vendor);
    println!("{}",&command);
    if settings.default_login {
        Command::new("cmd")
        .args(["/K", &command])
        .spawn() 
        .expect("Erro ao abrir a loja");
    }   
}
pub fn open_project(project_path: &Path) {
    let dir_list = get_child_dirs(&project_path);
    let path_splited = project_path.to_str().unwrap().split("\\");
    let actual_vendor = path_splited.last().unwrap(); 
    login_vendor(&actual_vendor);
    for dir in dir_list.into_iter() {
        open_vscode(&dir);
    }
}

pub fn get_child_dirs(dir: &Path) -> Vec<String> {
    let child = std::fs::read_dir(dir);
    let mut folders: Vec<String> = Vec::new();
    match child {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Some(str) = entry.path().to_str() {
                            folders.push(str.to_string())
                        }
                    }
                    Err(e) => println!("deu erro dentro {}", e),
                }
            }
        }
        Err(e) => println!("deu erro {}", e),
    }
    return folders;
}
