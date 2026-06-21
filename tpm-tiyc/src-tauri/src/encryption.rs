use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use chrono::{NaiveDate, Local};

// Magic bytes for .cufa
const CUFA_MAGIC: &[u8; 4] = b"CUFA";

// Shared state to hold pending encryption configuration.
pub struct EncryptionState {
    pub pending: Mutex<Option<EncryptionConfig>>,
}

#[derive(Clone)]
pub struct EncryptionConfig {
    pub original_file_path: String,
    pub target_date: String,
    pub strict_date: bool,
}

#[tauri::command]
pub fn set_encryption_settings(
    state: tauri::State<EncryptionState>,
    original_path: String,
    target_date: String,
    strict_date: bool,
) -> Result<(), String> {
    let mut pending = state.pending.lock().map_err(|e| e.to_string())?;
    *pending = Some(EncryptionConfig {
        original_file_path: original_path,
        target_date,
        strict_date,
    });
    Ok(())
}

#[tauri::command]
pub fn clear_encryption_settings(state: tauri::State<EncryptionState>) -> Result<(), String> {
    let mut pending = state.pending.lock().map_err(|e| e.to_string())?;
    *pending = None;
    Ok(())
}

#[tauri::command]
pub fn close_workspace(state: tauri::State<EncryptionState>) -> Result<(), String> {
    let mut pending = state.pending.lock().map_err(|e| e.to_string())?;
    
    println!("close_workspace called! pending is some? {}", pending.is_some());
    if let Some(config) = pending.take() {
        println!("Config original path: {}", config.original_file_path);
        let original_path = Path::new(&config.original_file_path);
        let parent = original_path.parent().unwrap_or(Path::new(""));
        let file_stem = original_path.file_stem().and_then(|s| s.to_str()).unwrap_or("encrypted");
        
        // Generate the output .cufa path (e.g., path/to/original_file.cufa)
        let cufa_name = format!("{}.cufa", file_stem);
        let cufa_path = parent.join(&cufa_name);
        
        println!("Generating .cufa at: {:?}", cufa_path);
        let _ = std::fs::write("debug.txt", format!("close_workspace called! original path: {}, parent: {:?}, cufa_path: {:?}", config.original_file_path, parent, cufa_path));
        match pack_workspace(&config.target_date, config.strict_date, &cufa_path) {
            Ok(_) => {
                let _ = std::fs::write("debug_success.txt", "Successfully packed workspace");
            },
            Err(e) => {
                let _ = std::fs::write("debug_error.txt", format!("Error packing workspace: {}", e));
            },
        }
    } else {
        let _ = std::fs::write("debug.txt", "close_workspace called but pending was None");
    }
    
    // Clear workspace directory contents regardless
    let workspace_dir = Path::new("workspace");
    if workspace_dir.exists() {
        if let Ok(entries) = fs::read_dir(workspace_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    let _ = fs::remove_dir_all(path);
                } else {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
    
    Ok(())
}

pub fn pack_workspace(target_date: &str, strict_date: bool, output_path: &Path) -> Result<(), String> {
    let workspace_dir = Path::new("workspace");
    if !workspace_dir.exists() {
        return Err("Workspace directory does not exist".to_string());
    }

    let mut out_file = File::create(output_path).map_err(|e| format!("Failed to create .cufa file: {}", e))?;
    
    // Write magic bytes
    out_file.write_all(CUFA_MAGIC).map_err(|e| e.to_string())?;
    
    // Write date length and string
    let date_bytes = target_date.as_bytes();
    out_file.write_all(&(date_bytes.len() as u32).to_le_bytes()).map_err(|e| e.to_string())?;
    out_file.write_all(date_bytes).map_err(|e| e.to_string())?;
    
    // Write strict_date boolean
    let strict_byte = if strict_date { 1u8 } else { 0u8 };
    out_file.write_all(&[strict_byte]).map_err(|e| e.to_string())?;
    
    // Gather files
    let mut files_to_pack = Vec::new();
    for entry in fs::read_dir(workspace_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let meta = entry.metadata().map_err(|e| e.to_string())?;
        if meta.is_file() {
            files_to_pack.push(entry.path());
        }
    }
    
    // Write number of files
    out_file.write_all(&(files_to_pack.len() as u32).to_le_bytes()).map_err(|e| e.to_string())?;
    
    for path in files_to_pack {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
        let name_bytes = name.as_bytes();
        
        // Write name length and name
        out_file.write_all(&(name_bytes.len() as u32).to_le_bytes()).map_err(|e| e.to_string())?;
        out_file.write_all(name_bytes).map_err(|e| e.to_string())?;
        
        // Read file contents
        let mut content = Vec::new();
        let mut file = File::open(&path).map_err(|e| e.to_string())?;
        file.read_to_end(&mut content).map_err(|e| e.to_string())?;
        
        // Write content length and content
        out_file.write_all(&(content.len() as u32).to_le_bytes()).map_err(|e| e.to_string())?;
        out_file.write_all(&content).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub fn load_encrypted_workspace(path: &str) -> Result<String, String> {
    let input_path = PathBuf::from(path);
    let mut file = File::open(&input_path).map_err(|e| format!("Failed to open .cufa file: {}", e))?;
    
    // Validate magic bytes
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic).map_err(|_| "Invalid .cufa file format")?;
    if &magic != CUFA_MAGIC {
        return Err("Invalid magic bytes".to_string());
    }
    
    // Read date
    let mut len_buf = [0u8; 4];
    file.read_exact(&mut len_buf).map_err(|_| "Failed to read date length")?;
    let date_len = u32::from_le_bytes(len_buf) as usize;
    let mut date_bytes = vec![0u8; date_len];
    file.read_exact(&mut date_bytes).map_err(|_| "Failed to read date")?;
    let target_date_str = String::from_utf8_lossy(&date_bytes).to_string();
    
    // Read strict flag
    let mut strict_buf = [0u8; 1];
    file.read_exact(&mut strict_buf).map_err(|_| "Failed to read strict flag")?;
    let strict_date = strict_buf[0] == 1;
    
    // Validate Date
    let target_date = NaiveDate::parse_from_str(&target_date_str, "%Y-%m-%d")
        .map_err(|_| "Invalid date format in .cufa file".to_string())?;
    
    let current_date = Local::now().date_naive();
    
    if strict_date {
        if current_date != target_date {
            return Err(format!("Este archivo está encriptado y solo puede abrirse el {}.", target_date_str));
        }
    } else {
        if current_date < target_date {
            return Err(format!("Este archivo está encriptado y recién podrá abrirse a partir del {}.", target_date_str));
        }
    }
    
    // Workspace validation passed. Recreate workspace.
    let workspace_dir = Path::new("workspace");
    if workspace_dir.exists() {
        let _ = fs::remove_dir_all(workspace_dir);
    }
    fs::create_dir(workspace_dir).map_err(|e| format!("Failed to create workspace: {}", e))?;
    
    // Read files
    let mut files_count_buf = [0u8; 4];
    file.read_exact(&mut files_count_buf).map_err(|_| "Failed to read files count")?;
    let files_count = u32::from_le_bytes(files_count_buf);
    
    let mut main_file_path = None;
    let mut backup_file_path = None;
    
    for _ in 0..files_count {
        // Read name
        let mut name_len_buf = [0u8; 4];
        file.read_exact(&mut name_len_buf).map_err(|_| "Failed to read name length")?;
        let name_len = u32::from_le_bytes(name_len_buf) as usize;
        let mut name_bytes = vec![0u8; name_len];
        file.read_exact(&mut name_bytes).map_err(|_| "Failed to read name")?;
        let file_name = String::from_utf8_lossy(&name_bytes).to_string();
        
        // Read content
        let mut content_len_buf = [0u8; 4];
        file.read_exact(&mut content_len_buf).map_err(|_| "Failed to read content length")?;
        let content_len = u32::from_le_bytes(content_len_buf) as usize;
        let mut content = vec![0u8; content_len];
        file.read_exact(&mut content).map_err(|_| "Failed to read content")?;
        
        let out_path = workspace_dir.join(&file_name);
        let mut out_file = File::create(&out_path).map_err(|e| format!("Failed to write file {}: {}", file_name, e))?;
        out_file.write_all(&content).map_err(|e| format!("Failed to write content to {}: {}", file_name, e))?;
        
        if let Ok(abs_path) = fs::canonicalize(&out_path) {
            let path_str = abs_path.to_string_lossy().into_owned();
            let is_generated = file_name.ends_with(".huf") 
                || file_name.contains(".HA") || file_name.contains(".HE") 
                || file_name.contains(".DC") || file_name.contains(".DE");
                
            if backup_file_path.is_none() {
                backup_file_path = Some(path_str.clone());
            }
            if !is_generated && main_file_path.is_none() {
                main_file_path = Some(path_str);
            }
        }
    }
    
    let selected_path = main_file_path.or(backup_file_path);
    
    // Return path to the main file (or whatever was loaded).
    if let Some(path) = selected_path {
        Ok(path)
    } else {
        if let Ok(abs_path) = fs::canonicalize(workspace_dir) {
            Ok(abs_path.to_string_lossy().into_owned())
        } else {
            Ok(workspace_dir.to_string_lossy().into_owned())
        }
    }
}
