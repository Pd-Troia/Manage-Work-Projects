use std::fs;
#[derive(serde::Deserialize)]
pub struct Setting{
    root_path: String
}
pub struct Config{
    setting: Setting
}

impl Config{    
    pub fn new(path : &String)-> Self{       
        let path = "./config.json".to_string();
        let file = fs::read_to_string(&path).expect("erro ao ler o arquivo de configuração");
        let str_file = file.as_str();
        let setting: Setting = serde_json::from_str(str_file).expect("erro na desserialização");
        Self { setting: setting }
    }    
}