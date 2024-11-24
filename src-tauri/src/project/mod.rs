use std::path::Path;
use std::process::Command;
fn open_vscode(){
    println!("maria");
    Command::new("code")
             .arg(".")
             .output()
             .expect("falha ao abrir vscode");
 }
 fn open_dir(dir:&String){
    println!("joao");
     Command::new("cmd")
             .args(["cd", dir])
             .output()  
             .expect("falha ao mudar o diretorio");
 }
pub fn open_project(project_path:&Path){
    let dir_list = get_child_dirs(project_path);
    for dir in dir_list.into_iter() {        
         open_dir(&dir);
         open_vscode();       
    }
}


pub fn get_child_dirs (dir : &Path)->Vec<String>{ 
    let child = std::fs::read_dir(dir);
    let mut folders: Vec<String> = Vec::new();    
    match child {
       Ok(entries)=>{
        for entry in entries {
            match entry {
                Ok(entry)=>{                   
                    if let Some(str) = entry.path().to_str(){
                        folders.push(str.to_string())
                    }                   
                }
                Err(e)=>println!("deu erro dentro {}",e)
            }
        }
       }
       Err(e)=> println!("deu erro {}",e)
    } 
    return folders  
}