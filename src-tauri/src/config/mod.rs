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

    pub fn new_with_path(path: &str) -> Self {
        let path_buf = std::path::PathBuf::from(path);
        if !path_buf.exists() {
            let default = String::from("{\"root_folder\":\"\",\"default_login\":false}");
            fs::write(&path_buf, default).expect("erro ao criar config de teste");
        }
        let str_file = Self::get_config_file(&path.to_string());
        let settings: Settings =
            serde_json::from_str(str_file.as_str()).expect("erro na desserialização");
        Self {
            path: path.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(suffix: &str) -> String {
        std::env::temp_dir()
            .join(format!("manage_projects_test_{}.json", suffix))
            .to_str()
            .unwrap()
            .to_string()
    }

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    // --- Config file creation ---

    #[test]
    fn test_config_criado_com_valores_padrao() {
        let path = temp_path("default_values");
        cleanup(&path);

        let config = Config::new_with_path(&path);
        let settings = config.get_settings();

        assert_eq!(settings.root_folder, "");
        assert_eq!(settings.default_login, false);

        cleanup(&path);
    }

    #[test]
    fn test_config_reutiliza_arquivo_existente() {
        let path = temp_path("reuse_existing");
        cleanup(&path);

        let json = r#"{"root_folder":"C:\\Projetos","default_login":true}"#;
        fs::write(&path, json).unwrap();

        let config = Config::new_with_path(&path);
        let settings = config.get_settings();

        assert_eq!(settings.root_folder, "C:\\Projetos");
        assert_eq!(settings.default_login, true);

        cleanup(&path);
    }

    // --- Leitura do AppData (save + get_settings) ---

    #[test]
    fn test_save_root_folder_persiste_no_disco() {
        let path = temp_path("save_root_folder");
        cleanup(&path);

        let mut config = Config::new_with_path(&path);
        config.save_root_folder("C:\\Projetos\\MeuVendor".to_string());

        let config2 = Config::new_with_path(&path);
        assert_eq!(
            config2.get_settings().root_folder,
            "C:\\Projetos\\MeuVendor"
        );

        cleanup(&path);
    }

    #[test]
    fn test_save_default_login_true_persiste_no_disco() {
        let path = temp_path("save_default_login_true");
        cleanup(&path);

        let mut config = Config::new_with_path(&path);
        config.save_default_login(true);

        let config2 = Config::new_with_path(&path);
        assert_eq!(config2.get_settings().default_login, true);

        cleanup(&path);
    }

    #[test]
    fn test_save_default_login_false_persiste_no_disco() {
        let path = temp_path("save_default_login_false");
        cleanup(&path);

        let mut config = Config::new_with_path(&path);
        config.save_default_login(true);
        config.save_default_login(false);

        let config2 = Config::new_with_path(&path);
        assert_eq!(config2.get_settings().default_login, false);

        cleanup(&path);
    }

    // --- Serialização / deserialização de Settings ---

    #[test]
    fn test_settings_desserializacao() {
        let json = r#"{"root_folder":"C:\\Projetos","default_login":true}"#;
        let settings: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.root_folder, "C:\\Projetos");
        assert_eq!(settings.default_login, true);
    }

    #[test]
    fn test_settings_roundtrip_serializacao() {
        let original = Settings {
            root_folder: "C:\\Projetos\\Vendor".to_string(),
            default_login: false,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.root_folder, original.root_folder);
        assert_eq!(restored.default_login, original.default_login);
    }
}
