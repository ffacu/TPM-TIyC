use std::fs::File;
use std::io::{self, Read, Write};
use std::convert::TryInto;

fn main() -> io::Result<()> {
    // 1. Open your input file
    let mut input = File::open("hola.txt")?;
    
    // 2. Create an output file for the results
    let mut output = File::create("output.txt")?;

    // 3. Define the block size for testing (change this to whatever you need)
    let block_size_bits: usize = 8; 

    // Read the entire file into a byte vector
    let mut buffer =  Vec::new();
    let mut codeword = Vec::new();

    let mut overall_parity = 0;

    input.read_to_end(&mut buffer)?; //read the file input into the buffer vector (read bytes not bits, miss the conversion)

    let mut bits_info: Vec<u8> = Vec::new();
    // Iteramos PRIMERO por los bytes, y LUEGO extraemos sus 8 bits
    for byte in &buffer {
        for b in 0..8 {
            bits_info.push((byte >> b) & 1);
        }
    }

    //calculate the required control bits for the block size
    let control_bits_quantity = block_size_bits.trailing_zeros() as usize;

    // 2. Calculamos los bits de información reales (Para 8 bits: 8 - 1 - 3 = 4)
    let info_bits_quantity = block_size_bits - 1 - control_bits_quantity;
    
    let mut internal_codeword = vec![0; (block_size_bits).try_into().unwrap()];    

    
    //one iteration is for hamminizing each file block (the amount of bits of information is taken), check if this correctly implemented!
    //the information is taken in bytes, no bits, this implementation is partcially incorrect
    // Cambiamos 'buffer' por 'bits_info'
    for i in (0..(bits_info.len())).step_by(info_bits_quantity) {

        let mut parity_bits = 0; 
        let mut step_buffer = i;    

        //initialize the control bits and add the info bits
        for j in 0..(info_bits_quantity - 1) as usize {

            
            if j == 1 << parity_bits{
                
                internal_codeword[j] = 0; //parity placeholder
                parity_bits += 1;

            }
            else{

                internal_codeword[j] = bits_info[step_buffer]; //information bit placeholder
                step_buffer += 1;
                overall_parity = overall_parity ^ internal_codeword[j];
            
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
            overall_parity = overall_parity ^ parity_value;
        }

        internal_codeword[block_size_bits - 1] = overall_parity;

        //The last parity bit check has not been calculated

        // append the currently block to the output file
        codeword.append(&mut internal_codeword);

        // The vector needs to be reinitialized.
        internal_codeword = vec![0; (block_size_bits).try_into().unwrap()]; 
    }

    let mut output_bytes: Vec<u8> = Vec::new();
    
    // Agrupamos de a 8 bits
    for chunk in codeword.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << i; // Reconstruimos el byte
        }
        output_bytes.push(byte);
    }
    
    // 3. Escribir los bytes empaquetados en el archivo de salida
    output.write_all(&output_bytes)?;    

    Ok(())
}
