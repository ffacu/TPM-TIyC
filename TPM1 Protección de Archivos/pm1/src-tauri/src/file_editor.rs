use std::io::{self, Read, Write};

pub fn hamming(block_size_bits: isize, input: &mut std::fs::File, output: &mut std::fs::File) -> io::Result<()> {
    // Read the entire file into a byte vector
    let mut buffer =  Vec::new();
    input.read_to_end(&mut buffer)?;

    let mut encoded_stream = Vec::new();

    match block_size_bits {
        8 => {
            // TODO: Implement 8-bit logic
            // 1. Read 4 bits (half a byte) of data from 'buffer'
            // 2. Calculate parity bits for positions 1, 2, and 4.
            // 3. Calculate the overall parity bit for position 0 (Extended Hamming).
            // 4. Pack the 4 data bits and 4 parity bits into a single u8.
            // 5. Push to 'encoded_stream'.
        },
        1024 => {
            // TODO: Implement 1024-bit logic
            // 1. Read 1013 bits of data.
            // 2. Insert parity bits at positions 1, 2, 4, 8 ... 512.
            // 3. Pack into 128 bytes.
        },
        16384 => {
            // TODO: Implement 16384-bit logic
            // 1. Read 16369 bits of data.
            // 2. Insert parity bits at powers of 2 up to 8192.
            // 3. Pack into 2048 bytes.
        },
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported block size. Must be 8, 1024, or 16384.",
            ))
        }
    }

    // Write the fully processed binary stream to the output file
    output.write_all(&encoded_stream)?;
    
    Ok(())
}

pub fn error_injection(data: &mut [u8]) {
    // Example of bitwise masking to inject an error
    // Using XOR (^) to flip the 3rd bit (00000100 in binary, which is 4 in decimal) of the first byte
    if !data.is_empty() {
        data[0] ^= 4; 
    }
}