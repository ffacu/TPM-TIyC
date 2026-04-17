mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use tpm_tiyc_lib::file_editor::*;

    // --- CONFIGURATION ---
    // It will create this directory relative to where `cargo test` it's runned.
    const TEST_DIR_PATH: &str = "test_output_files";

    /// Helper function to create the testing directory and file paths
    fn setup_test_env(test_name: &str) -> (PathBuf, PathBuf, PathBuf) {
        // Create the directory if it doesn't exist
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
            "The decoded text does not match the original text!"
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

    #[test]
    fn test_hamming_16384_bit_lossless() {
        let (input_path, encoded_path, decoded_path) = setup_test_env("test_16384_bit");
        
        // Creating a larger payload to properly test the 16384-bit block size
        let original_string = "Laboratorio de Proteccion de Archivos 2026. ".repeat(60);
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
            hamming_encoding(16384, &mut input_file, &mut encoded_file).expect("Encoding failed");
        }

        // 3. Decode
        {
            let mut encoded_file = File::open(&encoded_path).unwrap();
            let mut decoded_file = File::create(&decoded_path).unwrap();
            hamming_decoding(16384, &mut encoded_file, &mut decoded_file).expect("Decoding failed");
        }

        // 4. Assertion
        let mut decoded_data = Vec::new();
        File::open(&decoded_path).unwrap().read_to_end(&mut decoded_data).unwrap();

        assert_eq!(
            original_data.to_vec(), 
            decoded_data, 
            "16384-bit block decoding failed to perfectly reconstruct the file."
        );
    }

    #[test]
    fn test_hamming_8_bit_error_correction() {
        let (input_path, encoded_path, decoded_path) = setup_test_env("test_8_bit_error_injection");
        let error_path = PathBuf::from(TEST_DIR_PATH).join("test_8_bit_error_injected.bin");
        let original_data = b"hola mundo"; 

        // 1. Setup
        {
            let mut input_file = File::create(&input_path).unwrap();
            input_file.write_all(original_data).unwrap();
        }

        // 2. Encode
        {
            let mut input_file = File::open(&input_path).unwrap();
            let mut encoded_file = File::create(&encoded_path).unwrap();
            hamming_encoding(8, &mut input_file, &mut encoded_file).expect("Encoding failed");
        }

        // 3. Inject Errors
        {
            let mut encoded_file = File::open(&encoded_path).unwrap();
            let mut error_file = File::create(&error_path).unwrap();
            inject_error(8, &mut encoded_file, &mut error_file).expect("Error injection failed");
        }

        // 4. Decode the file that has errors!
        {
            let mut error_file = File::open(&error_path).unwrap();
            let mut decoded_file = File::create(&decoded_path).unwrap();
            hamming_decoding(8, &mut error_file, &mut decoded_file).expect("Decoding failed");
        }

        // 5. Assertion
        let mut decoded_data = Vec::new();
        File::open(&decoded_path).unwrap().read_to_end(&mut decoded_data).unwrap();

        assert_eq!(
            original_data.to_vec(), 
            decoded_data, 
            "Error correction failed! The decoded text after error injection does not match the original."
        );
    }

    #[test]
    fn test_hamming_1024_bit_error_correction() {
        let (input_path, encoded_path, decoded_path) = setup_test_env("test_1024_bit_error_injection");
        let error_path = PathBuf::from(TEST_DIR_PATH).join("test_1024_bit_error_injected.bin");
        
        let original_string = "Probando inyeccion de error con 1024 bits. ".repeat(20);
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
            hamming_encoding(1024, &mut input_file, &mut encoded_file).unwrap();
        }

        // 3. Inject Errors
        {
            let mut encoded_file = File::open(&encoded_path).unwrap();
            let mut error_file = File::create(&error_path).unwrap();
            inject_error(1024, &mut encoded_file, &mut error_file).unwrap();
        }

        // 4. Decode
        {
            let mut error_file = File::open(&error_path).unwrap();
            let mut decoded_file = File::create(&decoded_path).unwrap();
            hamming_decoding(1024, &mut error_file, &mut decoded_file).unwrap();
        }

        // 5. Assertion
        let mut decoded_data = Vec::new();
        File::open(&decoded_path).unwrap().read_to_end(&mut decoded_data).unwrap();

        assert_eq!(
            original_data.to_vec(), 
            decoded_data, 
            "1024-bit Error correction failed!"
        );
    }

    #[test]
    fn test_hamming_16384_bit_error_correction() {
        let (input_path, encoded_path, decoded_path) = setup_test_env("test_16384_bit_error_injection");
        let error_path = PathBuf::from(TEST_DIR_PATH).join("test_16384_bit_error_injected.bin");
        
        let original_string = "Probando inyeccion de error con 16384 bits. ".repeat(60);
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
            hamming_encoding(16384, &mut input_file, &mut encoded_file).unwrap();
        }

        // 3. Inject Errors
        {
            let mut encoded_file = File::open(&encoded_path).unwrap();
            let mut error_file = File::create(&error_path).unwrap();
            inject_error(16384, &mut encoded_file, &mut error_file).unwrap();
        }

        // 4. Decode
        {
            let mut error_file = File::open(&error_path).unwrap();
            let mut decoded_file = File::create(&decoded_path).unwrap();
            hamming_decoding(16384, &mut error_file, &mut decoded_file).unwrap();
        }

        // 5. Assertion
        let mut decoded_data = Vec::new();
        File::open(&decoded_path).unwrap().read_to_end(&mut decoded_data).unwrap();

        assert_eq!(
            original_data.to_vec(), 
            decoded_data, 
            "16384-bit Error correction failed!"
        );
    }

}