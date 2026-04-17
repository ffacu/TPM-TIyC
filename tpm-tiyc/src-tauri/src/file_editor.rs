use std::{io::{self, Read, Write}, process::exit};
use rand::Rng;

pub fn hamming_encoding(block_size_bits: usize, input: &mut std::fs::File, output: &mut std::fs::File) -> io::Result<()> {
    
    // Read the entire file into a byte vector
    let mut buffer =  Vec::new();
    let mut codeword = Vec::new();

    let mut overall_parity = 0;

    input.read_to_end(&mut buffer)?; //read the file input into the buffer vector

    let mut bits_info: Vec<u8> = Vec::new();
   
    // Cast byte to bits 
    for byte in &buffer {
        for b in 0..8 {
            bits_info.push((byte >> b) & 1);
        }
    }

    // Calculate the required control bits for the block size
    let control_bits_quantity = block_size_bits.trailing_zeros() as usize;

    // Calculate the required information bits for the block size
    let info_bits_quantity = block_size_bits - 1 - control_bits_quantity;
    
    let mut internal_codeword = vec![0; (block_size_bits).try_into().unwrap()];    

    let missed_bits = bits_info.len() % info_bits_quantity;
    
    if missed_bits != 0 {
        let padding_needed = info_bits_quantity - missed_bits; 
        for _i in 0..padding_needed {
            bits_info.push(0);
        }
    }
    
    // One iteration is for hamminizing each file block
    for i in (0..(bits_info.len())).step_by(info_bits_quantity) {

        let mut parity_bits = 0;
        let mut step_buffer = i;

        // Initialize the control bits and add the info bits
        for j in 1..(block_size_bits + 1) as usize {

            
            if j == (1 << parity_bits) {
                
                internal_codeword[j - 1] = 0; // Parity placeholder
                parity_bits += 1;

            }
            else {

                internal_codeword[j - 1] = bits_info[step_buffer]; // Information bit placeholder
                step_buffer += 1;
                overall_parity = overall_parity ^ internal_codeword[j - 1];
            
            }
        }

        // Calculate Hamming control bits
        for j in 0..(control_bits_quantity) {
            let parity_position: usize = 1 << j;
            let mut parity_value = 0;

            for bit_position in 1..(block_size_bits) {
                
                if (bit_position & parity_position) != 0 {
    
                    parity_value = parity_value ^ internal_codeword[bit_position - 1];
                    
                }
                
            }

            internal_codeword[parity_position - 1] = parity_value;
            overall_parity = overall_parity ^ parity_value;
        }

        internal_codeword[block_size_bits - 1] = overall_parity; // Overall parity check bit


        // Append the currently block to the output file
        codeword.append(&mut internal_codeword);

        // The vector needs to be reinitialized.
        internal_codeword = vec![0; (block_size_bits).try_into().unwrap()]; 
    }

    let mut output_bytes: Vec<u8> = Vec::new();
    
    // Bits packaged into bytes
    for chunk in codeword.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << i; // Rebuild byte
        }
        output_bytes.push(byte);
    }
    
    // Drop the output into file
    output.write_all(&output_bytes)?;    

    Ok(())

}


pub fn hamming_decoding(block_size_bits: usize, input: &mut std::fs::File, output: &mut std::fs::File) -> io::Result<()>  {
   
    let mut buffer =  Vec::new();
    let mut word = Vec::new();

    input.read_to_end(&mut buffer)?; // Read the file input into the buffer vector

    let mut bits_info: Vec<u8> = Vec::new();
    
    // Cast byte to bits 
    for byte in &buffer {
        for b in 0..8 {
            bits_info.push((byte >> b) & 1); // Put bits into buffer aux
        }
    }

    let control_bits_quantity = block_size_bits.trailing_zeros() as usize;
    let mut bits_info_internal = vec![0; (block_size_bits).try_into().unwrap()];  
    
    // The loop takes from the buffer the amount of the block
    for i in (0..(bits_info.len())).step_by(block_size_bits) {

        let mut step_buffer = i;

        for j in 0..(block_size_bits) as usize {

            bits_info_internal[j] = bits_info[step_buffer];
            step_buffer += 1; 

        }

        let mut overall_parity = 0;
        let mut syndrome = 0;
        let mut parity_value;

        // Calculate the syndrome of hamming block (n - 1 bits)
        for j in 0..(control_bits_quantity) {
            let parity_position = 1 << j;
            parity_value = 0;

            for bit_position in 1..(block_size_bits + 1) {

                if (bit_position & parity_position) != 0 {
                    parity_value = parity_value ^ bits_info_internal[bit_position - 1]; 
                } 
                overall_parity = overall_parity ^ bits_info_internal[bit_position - 1];
            
            }

            if parity_value != 0 {
                syndrome = syndrome + parity_position;
            }
        }

        // Correct the error if there is one.
        if syndrome == 0 {

            if overall_parity == 1 {
            
                bits_info_internal[block_size_bits - 1] ^= 1;
                
            }
            
        }
        else {
            
            if overall_parity == 0 {
                panic!("Se detectaron 2 (o mas) errores. El programa termina...")
            }
            else{
                bits_info_internal[syndrome - 1] ^= 1;
            }
        
        }

        // Append currently hamming block into vector (Forma pedorra)
        for i in 0..(control_bits_quantity + 1) {
            
            let parity_position = 1 << i;

            bits_info_internal[parity_position - 1] = 4;

        }

        for i in 0..block_size_bits {

            if bits_info_internal[i] != 4 {
                word.push(bits_info_internal[i]);
            }

        }

    }

    let mut output_bytes: Vec<u8> = Vec::new();
    
    // Bits packaged into byte
    for chunk in word.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << i; // Rebuild byte
        }
        output_bytes.push(byte);
    }
    
    output.write_all(&output_bytes)?;

    Ok(())

}

pub fn inject_error(block_size_bits: usize, input: &mut std::fs::File, output: &mut std::fs::File) -> io::Result<()> {
    
    // File open procedure
    let mut buffer =  Vec::new();
    input.read_to_end(&mut buffer)?; //read the file input into the buffer vector
    let mut bits_info: Vec<u8> = Vec::new();
    for byte in &buffer { // Cast byte to bits 
        for b in 0..8 {
            bits_info.push((byte >> b) & 1);
        }
    }

    // Index position selected by a random value
    
    // Introuce error
    let blocks_quantity = bits_info.len() / block_size_bits;
    for i in 0..blocks_quantity {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.5 {
            
            let index = rand::thread_rng().gen_range(0..block_size_bits);
            bits_info[index + i*block_size_bits] ^= 1;

        }
    }

    // Edit output file
    let mut output_bytes: Vec<u8> = Vec::new();
    
    // Bits packaged into byte
    for chunk in bits_info.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << i; // Rebuild byte
        }
        output_bytes.push(byte);
    }
    
    output.write_all(&output_bytes)?;

    Ok(())

}


// Test
#[cfg(test)]
mod tests {
    
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use std::path::PathBuf;

    use crate::file_editor::{hamming_decoding, hamming_encoding, inject_error};

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
mod tests {
    
    use super::*; // Imports your hamming_encoding, hamming_decoding, and inject_error functions
    use std::fs::{self, File};
    use std::io::Read;
    use std::path::PathBuf;

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