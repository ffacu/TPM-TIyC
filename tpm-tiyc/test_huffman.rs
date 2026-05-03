use std::fs;
use std::path::PathBuf;

mod huffman {
    include!("src-tauri/src/huffman/mod.rs");
}

fn main() {
    let input = PathBuf::from("workspace/texto_prueba.txt");
    let output_chars = PathBuf::from("workspace/texto_prueba_chars.huf");
    let extracted_chars = PathBuf::from("workspace/texto_prueba_chars.extracted.txt");

    // We assume the file exists, or we write some dummy data with rare chars
    fs::write(&input, "Hello \x0C Form Feed \r\n Windows \t Tab □ box!").unwrap();

    // Mode::Chars
    huffman::compress_file(input.clone(), output_chars.clone(), huffman::Mode::Chars).unwrap();
    huffman::extract_file(output_chars.clone(), extracted_chars.clone(), huffman::Mode::Chars).unwrap();

    let orig = fs::read(&input).unwrap();
    let ext_chars = fs::read(&extracted_chars).unwrap();

    println!("Chars mode lossless: {}", orig == ext_chars);
}
