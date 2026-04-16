#[cfg(test)]
mod tests {
    use super::*; // Imports your hamming_encoding and hamming_decoding functions
    use std::fs::{self, File};
    use std::io::Read;
    use std::path::PathBuf;

    // --- CONFIGURATION ---
    // Change this string to choose where test files are generated. 
    // It will create this directory relative to where you run `cargo test`.
    const TEST_DIR_PATH: &str = "test_output_files";

    /// Helper function to create the testing directory and file paths
    fn setup_test_env(test_name: &str) -> (PathBuf, PathBuf, PathBuf) {
        // Create the directory if it doesn't exist (Ubuntu/Linux safe)
        fs::create_dir_all(TEST_DIR_PATH).expect("Failed to create test directory");
        
        let base_path = PathBuf::from(TEST_DIR_PATH);
        let input_path = base_path.join(format!("{}_input.txt", test_name));
        let encoded_path = base_path.join(format!("{}_encoded.bin", test_name));
        let decoded_path = base_path.join(format!("{}_decoded.txt", test_name));

        (input_path, encoded_path, decoded_path)
    }

    #[test]
    fn test_hamming_8_bit_lossless() {
        let (input_path, encoded_path, decoded_path) = setup_test_env("test_8_bit");
        let original_data = b"hola mundo"; // 10 bytes of data

        // 1. Setup: Write the original text to the input file
        {
            let mut input_file = File::create(&input_path).expect("Could not create input file");
            input_file.write_all(original_data).unwrap();
        }

        // 2. Execution: Encode the file
        {
            let mut input_file = File::open(&input_path).unwrap();
            let mut encoded_file = File::create(&encoded_path).unwrap();
            hamming_encoding(8, &mut input_file, &mut encoded_file).expect("Encoding failed");
        }

        // 3. Execution: Decode the file
        {
            let mut encoded_file = File::open(&encoded_path).unwrap();
            let mut decoded_file = File::create(&decoded_path).unwrap();
            hamming_decoding(8, &mut encoded_file, &mut decoded_file).expect("Decoding failed");
        }

        // 4. Assertion: Read the decoded file and compare it to the original
        let mut decoded_data = Vec::new();
        File::open(&decoded_path).unwrap().read_to_end(&mut decoded_data).unwrap();

        assert_eq!(
            original_data.to_vec(), 
            decoded_data, 
            "The decoded text does not match the original text! Check your bit alignment."
        );
    }

    #[test]
    fn test_hamming_1024_bit_lossless() {
        let (input_path, encoded_path, decoded_path) = setup_test_env("test_1024_bit");
        
        // Creating a larger payload to properly test the 1024-bit block size
        let original_string = "Laboratorio de Proteccion de Archivos 2026. ".repeat(50);
        let original_data = original_string.as_bytes();

        // 1. Setup
        {
            let mut input_file = File::create(&input_path).unwrap();
            input_file.write_all(original_data).unwrap();
        }

        // 2. Encode
        {
            let mut input_file = File::open(&input_path).unwrap();
            let mut encoded_file = File::create(&encoded_path).unwrap();
            hamming_encoding(1024, &mut input_file, &mut encoded_file).expect("Encoding failed");
        }

        // 3. Decode
        {
            let mut encoded_file = File::open(&encoded_path).unwrap();
            let mut decoded_file = File::create(&decoded_path).unwrap();
            hamming_decoding(1024, &mut encoded_file, &mut decoded_file).expect("Decoding failed");
        }

        // 4. Assertion
        let mut decoded_data = Vec::new();
        File::open(&decoded_path).unwrap().read_to_end(&mut decoded_data).unwrap();

        assert_eq!(
            original_data.to_vec(), 
            decoded_data, 
            "1024-bit block decoding failed to perfectly reconstruct the file."
        );
    }
}