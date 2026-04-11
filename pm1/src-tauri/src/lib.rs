// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod greet_module;
mod file_handler;
mod file_editor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet_module::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//[file_hanlder::file_editor::hamming]
//[file_hanlder::file_editor::error_injection]