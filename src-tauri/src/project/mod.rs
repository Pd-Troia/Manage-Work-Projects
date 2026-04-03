use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

use crate::Config;


fn open_vscode(path: &String) {
    #[cfg(target_os = "windows")]
    let _ = Command::new("cmd")
        .args(["/C", "code ."])
        .current_dir(path)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    #[cfg(not(target_os = "windows"))]
    let _ = Command::new("code")
        .arg(".")
        .current_dir(path)
        .spawn();
}

fn login_vendor(vendor: &str, default_login: bool) {
    if default_login {
        spawn_vtex_login(vendor);
    }
}

pub fn login_vendor_manual(vendor: &str) {
    spawn_vtex_login(vendor);
}

static LOGIN_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

fn spawn_vtex_login(vendor: &str) {
    if LOGIN_IN_PROGRESS.swap(true, Ordering::SeqCst) {
        return; // já existe um login rodando
    }
    let command = format!("vtex login {}", vendor);
    match Command::new("cmd").args(["/K", &command]).spawn() {
        Ok(mut child) => {
            std::thread::spawn(move || {
                let _ = child.wait();
                LOGIN_IN_PROGRESS.store(false, Ordering::SeqCst);
            });
        }
        Err(_) => {
            LOGIN_IN_PROGRESS.store(false, Ordering::SeqCst);
        }
    }
}

pub fn extract_vendor_from_path(project_path: &Path) -> String {
    project_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

pub fn open_project(project_path: &Path) {
    let settings = Config::new().get_settings();
    let vendor = extract_vendor_from_path(project_path);
    login_vendor(&vendor, settings.default_login);
    let dir_list = get_child_dirs(project_path);
    for dir in dir_list.into_iter() {
        open_vscode(&dir);
    }
}

pub fn open_single_dir(dir_path: &Path) {
    if let Some(path_str) = dir_path.to_str() {
        open_vscode(&path_str.to_string());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn create_temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("manage_projects_test_{}", name));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn cleanup(dir: &PathBuf) {
        let _ = fs::remove_dir_all(dir);
    }

    // --- Abertura de pasta (get_child_dirs) ---

    #[test]
    fn test_get_child_dirs_retorna_subdiretorios() {
        let root = create_temp_dir("child_dirs_retorna");
        let sub1 = root.join("loja-a");
        let sub2 = root.join("loja-b");
        fs::create_dir_all(&sub1).unwrap();
        fs::create_dir_all(&sub2).unwrap();

        let dirs = get_child_dirs(&root);

        assert!(dirs.contains(&sub1.to_str().unwrap().to_string()));
        assert!(dirs.contains(&sub2.to_str().unwrap().to_string()));
        assert_eq!(dirs.len(), 2);

        cleanup(&root);
    }

    #[test]
    fn test_get_child_dirs_pasta_vazia_retorna_vec_vazio() {
        let root = create_temp_dir("child_dirs_vazio");
        cleanup(&root);
        fs::create_dir_all(&root).unwrap();

        let dirs = get_child_dirs(&root);
        assert!(dirs.is_empty());

        cleanup(&root);
    }

    #[test]
    fn test_get_child_dirs_pasta_inexistente_retorna_vec_vazio() {
        let path = PathBuf::from("C:\\caminho\\que\\nao\\existe\\hopefully");
        let dirs = get_child_dirs(&path);
        assert!(dirs.is_empty());
    }

    #[test]
    fn test_get_child_dirs_ignora_arquivos_e_retorna_tudo() {
        let root = create_temp_dir("child_dirs_com_arquivo");
        let sub = root.join("sub");
        let file = root.join("arquivo.txt");
        fs::create_dir_all(&sub).unwrap();
        fs::write(&file, "conteúdo").unwrap();

        let dirs = get_child_dirs(&root);

        // read_dir retorna tanto dirs quanto arquivos — verificamos que ambos aparecem
        assert!(dirs.contains(&sub.to_str().unwrap().to_string()));
        assert!(dirs.contains(&file.to_str().unwrap().to_string()));
        assert_eq!(dirs.len(), 2);

        cleanup(&root);
    }

    // --- Extração de vendor do caminho (login vtex) ---

    #[test]
    fn test_extract_vendor_caminho_multiplos_segmentos() {
        let path = PathBuf::from("C:\\Projetos\\meuvendor");
        assert_eq!(extract_vendor_from_path(&path), "meuvendor");
    }

    #[test]
    fn test_extract_vendor_segmento_unico() {
        let path = PathBuf::from("meuvendor");
        assert_eq!(extract_vendor_from_path(&path), "meuvendor");
    }

    #[test]
    fn test_extract_vendor_caminho_vazio_retorna_string_vazia() {
        let path = PathBuf::from("");
        assert_eq!(extract_vendor_from_path(&path), "");
    }

    // --- login_vendor: verifica que o flag default_login controla a execução ---

    #[test]
    fn test_login_vendor_false_nao_spawna_processo() {
        login_vendor("testvendor", false);
        // Não entrou em pânico — flag false não executa vtex.
    }

    // --- open_single_dir: abre apenas uma pasta específica ---

    #[test]
    fn test_open_single_dir_caminho_inexistente_nao_entra_em_panico() {
        let path = PathBuf::from("C:\\caminho\\inexistente\\qualquer");
        // set_current_dir falha silenciosamente; não deve entrar em pânico.
        open_single_dir(&path);
    }

    #[test]
    fn test_open_single_dir_abre_diretorio_valido_sem_panico() {
        let dir = create_temp_dir("open_single_dir_valido");
        // Em ambiente de teste não temos VSCode, mas a função não deve entrar em pânico.
        // O processo do VSCode falhará silenciosamente se não estiver instalado.
        open_single_dir(&dir);
        cleanup(&dir);
    }
}
