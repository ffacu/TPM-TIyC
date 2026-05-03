use std::fs::File;
use std::path::Path;
use crate::file_hamming;
use crate::huffman::{compress_file, extract_file, Mode};
use std::path::PathBuf;

#[tauri::command]
pub fn protect_file(path: &str, block_size_opt: u8, errors_quantity: usize) -> Result<Vec<String>, String> {
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

    // Generate new path for the protected file.
    let input_path = Path::new(path);
    let parent_dir = input_path.parent().unwrap_or(Path::new("")); // Get the parent directory.
    let file_stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output"); // Get the file stem.

    let ha_filename = format!("{}.{}", file_stem, ext); //Put the extension to the file name.
    let ha_path = parent_dir.join(&ha_filename); 

    let mut input_file = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
    let mut out_ha = File::create(&ha_path).map_err(|e| format!("Failed to create HA file: {}", e))?;

    file_hamming::hamming_encoding(block_size_bits, &mut input_file, &mut out_ha)
        .map_err(|e| format!("Hamming encoding failed: {}", e))?;

    let mut generated_files = vec![ha_filename.clone()];

    // If inject errors is true, create HE file.
    if errors_quantity > 0 {
        let he_filename = format!("{}.{}", file_stem, err_ext);
        let he_path = parent_dir.join(&he_filename);
        
        let mut ha_read = File::open(&ha_path).map_err(|e| format!("Failed to open HA file for reading: {}", e))?;
        let mut out_he = File::create(&he_path).map_err(|e| format!("Failed to create HE file: {}", e))?;
        
        file_hamming::inject_error(block_size_bits, errors_quantity, &mut ha_read, &mut out_he)
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
        // Create both .DCx and .DEx files.
        let dc_filename = format!("{}.DC{}", file_stem, block_idx);
        let dc_path = parent_dir.join(&dc_filename);
        let mut input_file1 = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
        let mut out_file1 = File::create(&dc_path).map_err(|e| format!("Failed to create DC file: {}", e))?;
        
        let decode_res_1 = file_hamming::hamming_decoding(block_size_bits, true, &mut input_file1, &mut out_file1);
        generated_files.push(dc_filename);

        // Create .DEx (With Errors)
        let de_filename = format!("{}.DE{}", file_stem, block_idx);
        let de_path = parent_dir.join(&de_filename);
        let mut input_file2 = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
        let mut out_file2 = File::create(&de_path).map_err(|e| format!("Failed to create DE file: {}", e))?;
        
        let decode_res_2 = file_hamming::hamming_decoding(block_size_bits, false, &mut input_file2, &mut out_file2);
        generated_files.push(de_filename);

        // Now return error if any of them failed
        if let Err(e) = decode_res_1 {
            return Err(format!("Hamming decoding (corrected) failed: {}", e));
        }
        if let Err(e) = decode_res_2 {
            return Err(format!("Hamming decoding (with error) failed: {}", e));
        }
    } else {
        // Create .DCx (No errors originally)
        let dec_filename = format!("{}.DC{}", file_stem, block_idx);
        let dec_path = parent_dir.join(&dec_filename);
        let mut input_file = File::open(path).map_err(|e| format!("Failed to open input file: {}", e))?;
        let mut out_file = File::create(&dec_path).map_err(|e| format!("Failed to create DC file: {}", e))?;
        
        file_hamming::hamming_decoding(block_size_bits, false, &mut input_file, &mut out_file)
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
    // Get the txt file name. 
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
    
    // Create a copy of the txt file in the workspace
    let dest_path = workspace_dir.join(file_name);
    fs::copy(input_path, &dest_path).map_err(|e| format!("Failed to copy file to workspace: {}", e))?;
    
    // Return the path of the copied file
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
    // Read all files in the workspace and append them to the list
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

#[tauri::command]
pub fn compress_huffman(path: &str, mode_str: &str) -> Result<String, String> {
    let mode = match mode_str {
        "words" => Mode::Words,
        "chars" => Mode::Chars,
        _ => return Err("Invalid mode. Use 'words' or 'chars'".to_string()),
    };
    
    let input_path = PathBuf::from(path);
    let mut output_path = input_path.clone();
    output_path.set_extension("huf");

    compress_file(input_path, output_path.clone(), mode)
        .map_err(|e| format!("Compression failed: {}", e))?;

    Ok(output_path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn extract_huffman(path: &str, mode_str: &str) -> Result<String, String> {
    let mode = match mode_str {
        "words" => Mode::Words,
        "chars" => Mode::Chars,
        _ => return Err("Invalid mode. Use 'words' or 'chars'".to_string()),
    };
    
    let input_path = PathBuf::from(path);
    let mut output_path = input_path.clone();
    output_path.set_extension("extracted.txt");

    extract_file(input_path, output_path.clone(), mode)
        .map_err(|e| format!("Extraction failed: {}", e))?;

    Ok(output_path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn get_file_size(path: &str) -> Result<u64, String> {
    std::fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| format!("Failed to get metadata: {}", e))
}