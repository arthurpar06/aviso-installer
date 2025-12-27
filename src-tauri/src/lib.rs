// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod aviso;

#[tauri::command]
fn install_aviso(lfxx_path: &str, aviso_path: &str) -> Result<(), String> {
    aviso::install_aviso(lfxx_path, aviso_path)
}

#[tauri::command]
fn install_aviso_content(lfxx_path: &str, aviso_content: &str) -> Result<(), String> {
    aviso::install_aviso_content(lfxx_path, aviso_content)
}

#[tauri::command]
fn check_aviso_installed(
    lfxx_path: &str,
    aviso_content: &str,
) -> Result<aviso::InstallationStatus, String> {
    aviso::is_aviso_installed(lfxx_path, aviso_content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            install_aviso,
            install_aviso_content,
            check_aviso_installed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
