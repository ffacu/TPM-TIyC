use std::fs::File;
use std::path::Path;
use crate::file_editor;

#[tauri::command]
pub fn protect_file(path: &str, block_size_opt: u8, inject_errors: bool) -> Result<Vec<String>, String> {
    // block_size_opt: 1 -> 8 bits (.HA1), 2 -> 1024 bits (.HA2), 3 -> 16384 bits (.HA3)
    let block_size_bits = match block_size_opt {
        1 => 8,
        2 => 1024,
        3 => 16384,
        _ => return Err("Invalid block size option".to_string()),
    };

    let ext = match block_size_opt {
        1 => "HA1",
        2 => "HA2",
        3 => "HA3",
        _ => unreachable!(),
    };

    let err_ext = match block_size_opt {
        1 => "HE1",
        2 => "HE2",
        3 => "HE3",
        _ => unreachable!(),
    };

    let input_path = Path::new(path);
    let parent_dir = input_path.parent().unwrap_or(Path::new(""));
    let file_stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");

    let ha_filename = format!("{}.{}", file_stem, ext);
    let ha_path = parent_dir.join(&ha_filename);

    let mut input_file = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
    let mut out_ha = File::create(&ha_path).map_err(|e| format!("Failed to create HA file: {}", e))?;

    file_editor::hamming_encoding(block_size_bits, &mut input_file, &mut out_ha)
        .map_err(|e| format!("Hamming encoding failed: {}", e))?;

    let mut generated_files = vec![ha_filename.clone()];

    if inject_errors {
        let he_filename = format!("{}.{}", file_stem, err_ext);
        let he_path = parent_dir.join(&he_filename);
        
        let mut ha_read = File::open(&ha_path).map_err(|e| format!("Failed to open HA file for reading: {}", e))?;
        let mut out_he = File::create(&he_path).map_err(|e| format!("Failed to create HE file: {}", e))?;
        
        file_editor::inject_error(block_size_bits, &mut ha_read, &mut out_he)
            .map_err(|e| format!("Error injection failed: {}", e))?;
            
        generated_files.push(he_filename);
    }

    Ok(generated_files)
}

#[tauri::command]
pub fn unprotect_file(path: &str) -> Result<Vec<String>, String> {
    let input_path = Path::new(path);
    let parent_dir = input_path.parent().unwrap_or(Path::new(""));
    let file_stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let ext = input_path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let mut generated_files = Vec::new();

    let block_size_bits = match ext {
        "HA1" | "HE1" => 8,
        "HA2" | "HE2" => 1024,
        "HA3" | "HE3" => 16384,
        _ => return Err("Invalid file extension for unprotected".to_string()),
    };

    let is_error_file = ext.starts_with("HE");
    let block_idx = ext.chars().last().unwrap_or('1');

    if is_error_file {
        // Create .DCx (Corrected)
        let dc_filename = format!("{}.DC{}", file_stem, block_idx);
        let dc_path = parent_dir.join(&dc_filename);
        let mut input_file1 = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
        let mut out_file1 = File::create(&dc_path).map_err(|e| format!("Failed to create DC file: {}", e))?;
        
        file_editor::hamming_decoding(block_size_bits, true, &mut input_file1, &mut out_file1)
            .map_err(|e| format!("Hamming decoding (corrected) failed: {}", e))?;
        generated_files.push(dc_filename);

        // Create .DEx (With Errors)
        let de_filename = format!("{}.DE{}", file_stem, block_idx);
        let de_path = parent_dir.join(&de_filename);
        let mut input_file2 = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
        let mut out_file2 = File::create(&de_path).map_err(|e| format!("Failed to create DE file: {}", e))?;
        
        file_editor::hamming_decoding(block_size_bits, false, &mut input_file2, &mut out_file2)
            .map_err(|e| format!("Hamming decoding (with error) failed: {}", e))?;
        generated_files.push(de_filename);
    } else {
        // Create .DCx (No errors originally)
        let dec_filename = format!("{}.DC{}", file_stem, block_idx);
        let dec_path = parent_dir.join(&dec_filename);
        let mut input_file = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
        let mut out_file = File::create(&dec_path).map_err(|e| format!("Failed to create DC file: {}", e))?;
        
        file_editor::hamming_decoding(block_size_bits, false, &mut input_file, &mut out_file)
            .map_err(|e| format!("Hamming decoding failed: {}", e))?;
        generated_files.push(dec_filename);
    }

    Ok(generated_files)
}

#[tauri::command]
pub fn read_file_content(path: &str) -> Result<String, String> {
    std::fs::read(path)
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub fn initialize_workspace(path: &str) -> Result<String, String> {
    use std::fs;
    
    let input_path = Path::new(path);
    let file_name = input_path.file_name().ok_or("Invalid file name")?;
    
    // Create workspace directory in the current working directory
    let workspace_dir = Path::new("workspace");
    if !workspace_dir.exists() {
        fs::create_dir(workspace_dir).map_err(|e| format!("Failed to create workspace: {}", e))?;
    } else {
        // Clear workspace
        let _ = fs::remove_dir_all(workspace_dir);
        fs::create_dir(workspace_dir).map_err(|e| format!("Failed to recreate workspace: {}", e))?;
    }
    
    let dest_path = workspace_dir.join(file_name);
    fs::copy(input_path, &dest_path).map_err(|e| format!("Failed to copy file to workspace: {}", e))?;
    
    // Return the absolute path of the copied file
    let abs_path = fs::canonicalize(&dest_path).map_err(|e| format!("Failed to canonicalize path: {}", e))?;
    Ok(abs_path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn list_workspace_files() -> Result<Vec<String>, String> {
    use std::fs;
    
    let workspace_dir = Path::new("workspace");
    if !workspace_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut files = Vec::new();
    for entry in fs::read_dir(workspace_dir).map_err(|e| format!("Failed to read workspace: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        if let Ok(name) = entry.file_name().into_string() {
            files.push(name);
        }
    }
    // Optional: Sort files alphabetically for better UI presentation
    files.sort();
    Ok(files)
}