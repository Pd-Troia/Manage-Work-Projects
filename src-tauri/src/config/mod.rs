use std::{env, fs, path::Path};

use serde::de::Expected;
#[derive(serde::Deserialize,serde::Serialize)]
pub struct Settings{
    root_folder: String
}
pub struct Config{
    path: String,
    settings: Settings
}

impl Config{    
    pub fn new()-> Self{          
        let path = env::current_dir()
            .expect("executável não encontrado");
        let path_str = String::from(path            
            .join("src/config/config.json")
            .to_str()
            .unwrap());
        print!("mario {}",path_str);
        let path = Path::new(&path_str);
        let file = fs::read_to_string(&path).expect("erro ao ler o arquivo de configuração");
        let str_file = file.as_str();
        let settings: Settings = serde_json::from_str(str_file).expect("erro na desserialização");
        Self {
            path:path_str,
            settings 
        }
    } 
    fn save_settings(&self){
        let seriealized = serde_json::to_string(&self.settings)
        .ok()
        .unwrap();
        fs::write(&self.path, seriealized).expect("falha ao escrever o arquivo");
    }  
    pub fn save_root_folder(&mut self,root_folder:String){        
        self.settings.root_folder = root_folder;
        self.save_settings();
    }
}