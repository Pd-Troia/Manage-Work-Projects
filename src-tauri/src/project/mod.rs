use std::path::Path;



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