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
                println!("Se detectaron 2 (o mas) errores. El programa termina...");
                exit(0);
            }
            
            bits_info_internal[syndrome - 1] ^= 1;
        
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