use std::fs::File;
use std::io::{self, Read, Write};
use std::convert::TryInto;

fn main() -> io::Result<()> {
    // 1. Open your input file
    let mut input = File::open("encode_out.txt")?;
    
    // 2. Create an output file for the results
    let mut output = File::create("decode_out.txt")?;

    // 3. Define the block size for testing (change this to whatever you need)
    let block_size_bits: usize = 8; 

        let mut buffer =  Vec::new();
    let mut word = Vec::new();

    input.read_to_end(&mut buffer)?; // Read the file input into the buffer vector (read bytes not bits, miss the conversion)

    let mut bits_info: Vec<u8> = Vec::new();
    
    // Cast byte to bits 
    for byte in &buffer {
        for b in 0..8 {
            bits_info.push((byte >> b) & 1); //drop bits into buffer aux
        }
    }

    let control_bits_quantity = block_size_bits.trailing_zeros() as usize;
    let mut bits_info_internal = vec![0; (block_size_bits).try_into().unwrap()];  
    
    // The loop takes from the buffer the amount of the block
    for i in (0..(bits_info.len())).step_by(block_size_bits){   //- >> for 

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
            
                bits_info_internal[block_size_bits - 1] = !bits_info_internal[block_size_bits - 1]
            
            }

        }
        else {

            bits_info_internal[syndrome - 1] = !bits_info_internal[syndrome - 1];
        
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
