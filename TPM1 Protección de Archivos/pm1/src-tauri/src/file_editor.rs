use std::io::{self, Read, Write};

pub fn hamming(block_size_bits: i32, input: &mut std::fs::File, output: &mut std::fs::File) -> io::Result<()> {
    
    // Read the entire file into a byte vector
    let mut buffer =  Vec::new();
    let mut codeword = Vec::new();

    input.read_to_end(&mut buffer)?; //read the file input into the buffer vector (read bytes not bits, miss the conversion)

    let mut control_bits_quantity = 0;  
    
    //calculate the required control bits for the block size
    while 1 << control_bits_quantity <= block_size_bits + control_bits_quantity + 1 {
        control_bits_quantity = control_bits_quantity + 1;
    }
    
    let info_bits_quantity = (block_size_bits - 1 - control_bits_quantity).try_into().unwrap();

    let mut internal_codeword = vec![0; (block_size_bits - 1).try_into().unwrap()];    
    
    //one iteration is for hamminizing each file block (the amount of bits of information is taken), check if this correctly implemented!
    //the information is taken in bytes, no bits, this implementation is partcially incorrect
    for i in (0..(buffer.len())).step_by(info_bits_quantity) { 

        let mut parity_bits = 0; 
        let mut step_buffer = i;    

        //initialize the control bits and add the info bits
        for j in 1..(info_bits_quantity - 1) as usize {

            
            if j == 1 << parity_bits{
                
                internal_codeword[j] = 0; //parity placeholder
                parity_bits += 1;

            }
            else{

                internal_codeword[j] = buffer[step_buffer]; //information bit placeholder
                step_buffer += 1;
            
            }
        }

        //calculate control bits
        for j in 0..(control_bits_quantity-1){
            let parity_position: usize = 1 << j;
            let mut parity_value = 0;

            for bit_position in 1..(block_size_bits as usize - 1){
                
                if (bit_position & parity_position) != 0 {
    
                    parity_value = parity_value ^ internal_codeword[bit_position];
    
                }
        
            }

            internal_codeword[parity_position] = parity_value;
        }

        //The last parity bit check has not been calculated

        // append the currently block to the output file
        codeword.append(&mut internal_codeword);

        // The vector needs to be reinitialized.

    }

    // the information needs to be packaged into byte vector
    
    // Write the fully processed binary stream to the output file
    output.write_all(&internal_codeword)?;
    
    Ok(())
}

pub fn error_injection(data: &mut [u8]) {
    // Example of bitwise masking to inject an error
    // Using XOR (^) to flip the 3rd bit (00000100 in binary, which is 4 in decimal) of the first byte
    if !data.is_empty() {
        data[0] ^= 4; 
    }
}