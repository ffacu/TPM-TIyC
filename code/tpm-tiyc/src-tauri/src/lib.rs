// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;

mod file_handler;
pub mod file_hamming;
pub mod huffman;
pub mod encryption;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(encryption::EncryptionState {
            pending: std::sync::Mutex::new(None),
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Destroyed => {
                let state = window.state::<encryption::EncryptionState>();
                let _ = encryption::close_workspace(state);
            }
            _ => {}
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            file_handler::protect_file,
            file_handler::unprotect_file,
            file_handler::read_file_content,
            file_handler::initialize_workspace,
            file_handler::list_workspace_files,
            file_handler::compress_huffman,
            file_handler::extract_huffman,
            file_handler::get_file_size,
            file_handler::clean_generated_files,
            encryption::set_encryption_settings,
            encryption::clear_encryption_settings,
            encryption::close_workspace,
            encryption::load_encrypted_workspace
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
