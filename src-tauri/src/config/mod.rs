use std::{env, fs, path::Path};

#[derive(serde::Deserialize,serde::Serialize)]
pub struct Settings{
    pub root_folder: String
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
        let str_file = Self::get_config_file(&path_str);
        let settings: Settings = serde_json::from_str(str_file.as_str()).expect("erro na desserialização");
        Self {
            path:path_str,
            settings 
        }
    } 
    pub fn get_settings(&self)-> Settings{        
        let str_file = Self::get_config_file(&self.path);
        let settings: Settings = serde_json::from_str(str_file.as_str()).expect("erro ao ler o arquivo de configurações");
        return settings
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
    fn get_config_file(path_str : &String)-> String{
        let path = Path::new(&path_str);
        let file = fs::read_to_string(&path).expect("erro ao ler o arquivo de configuração");
        return file;
    }
}