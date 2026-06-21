pub mod compression;
pub mod freqs;
pub mod huffman;

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

// Debug: adds {:?} for debugging and inspection
// Clone  adds .clone() method
// Copy adds .copy() method (for simple types like enums)
// PartialEq and Eq: adds == and != operators
// Eq: allows to compare two trees for equality (used in tests)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Words,
    Chars,
}

pub fn get_base_stem(file_name: &str) -> &str {
    let lowercase = file_name.to_lowercase();
    let mut first_idx = file_name.len();
    
    for suffix in &[".huf", ".ha", ".he", ".dc", ".de"] {
        if let Some(idx) = lowercase.find(suffix) {
            if idx < first_idx {
                first_idx = idx;
            }
        }
    }
    
    &file_name[..first_idx]
}

pub fn compress_file(input: PathBuf, output: PathBuf, mode: Mode) -> Result<(), Box<dyn std::error::Error>> {
    let original_extension = input.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("txt")
        .to_string();

    let compressed = match mode {
        Mode::Words => {
            let text = fs::read_to_string(&input).map_err(|_| "El archivo no es texto válido y no se puede compactar por palabras. Por favor, usa 'Por Caracteres' para archivos binarios o protegidos.")?;
            let lines: Vec<_> = text.split('\n').map(|x| x.to_string()).collect();
            compression::compress(&lines, freqs::word_frequencies, |line| {
                line.split(' ').map(|token| token.to_string())
            }, original_extension)
        },
        Mode::Chars => {
            let data = fs::read(&input)?;
            compression::compress_bytes(&data, original_extension)
        }
    }?;

    fs::write(&output, compressed)?;
    Ok(())
}

pub fn extract_file(input: PathBuf, mode: Mode) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let data = fs::read(&input)?;

    let (content_bytes, original_extension) = match compression::extract_bytes(&data) {
        Ok(res) => res,
        Err(_) => {
            // If extracting as bytes fails, try extracting as words
            match compression::extract(&data, |tokens: Vec<String>| tokens.join(" ")) {
                Ok((lines, ext)) => (lines.join("\n").into_bytes(), ext),
                Err(e) => return Err(format!("Extraction failed. Make sure the file is a valid .huf file. Details: {}", e).into()),
            }
        }
    };

    let parent_dir = input.parent().unwrap_or(Path::new(""));
    let file_name = input.file_name().and_then(|s| s.to_str()).unwrap_or("output");
    
    let base_stem = get_base_stem(file_name);
    let output_filename = format!("{}_huf.{}", base_stem, original_extension);
    let output_path = parent_dir.join(output_filename);

    fs::write(&output_path, content_bytes)?;
    Ok(output_path)
}
