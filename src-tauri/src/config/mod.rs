use std::{env, fs, path::Path};

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug)]
pub struct Settings {
    pub root_folder: String,
    pub default_login: bool
}
#[derive(Debug)]
pub struct Config {
    path: String,
    settings: Settings,
}

impl Config {
    pub fn new() -> Self {
        let path_str = Self::create_if_not_exists_config_file();
        let str_file = Self::get_config_file(&path_str);
        let settings: Settings =
            serde_json::from_str(str_file.as_str()).expect("erro na desserialização");
        Self {
            path: path_str,
            settings,
        }
    }
    pub fn get_settings(&self) -> Settings {
        let str_file = Self::get_config_file(&self.path);
        let settings: Settings = serde_json::from_str(str_file.as_str())
            .expect("erro ao ler o arquivo de configurações");
        return settings;
    }
    fn save_settings(&self) {
        let seriealized = serde_json::to_string(&self.settings).ok().unwrap();
        fs::write(&self.path,seriealized).expect("falha ao escrever o arquivo");
    }
    pub fn save_root_folder(&mut self, root_folder: String) {
        self.settings.root_folder = root_folder;
        self.save_settings();
    }
    pub fn save_default_login(&mut self, default_login:bool){
        self.settings.default_login = default_login;
        self.save_settings();
    }
    fn get_config_file(path_str: &String) -> String {
        let path = Path::new(&path_str);
        let file = fs::read_to_string(&path).expect("erro ao ler o arquivo de configuração");
        return file;
    }
    fn create_if_not_exists_config_folder() -> std::path::PathBuf {
        let config_path = env::var("APPDATA").unwrap();
        let config_folder_path = Path::new(&config_path).join("Manage Projects");
        if let Err(_) = fs::create_dir(&config_folder_path) {
            return config_folder_path;
        }
        return config_folder_path;
    }
    fn create_if_not_exists_config_file() -> String {
        let config_folder = Self::create_if_not_exists_config_folder().join("config.json");
        let str_config_folder = String::from(config_folder.to_str().unwrap());
        if !config_folder.exists() {
            let str_file = String::from("{\"root_folder\":\"\",\"default_login\":false}");
            fs::write(config_folder, str_file)
                .ok()
                .expect("erro ao escrever o arquivo de configurações");
        }
        return str_config_folder;
    }
}
