// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod file_handler;
pub mod file_editor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            file_handler::protect_file,
            file_handler::unprotect_file,
            file_handler::read_file_content,
            file_handler::initialize_workspace,
            file_handler::list_workspace_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
