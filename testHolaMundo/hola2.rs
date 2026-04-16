use std::fs::File;
use std::io::{self, Read, Write};
use std::convert::TryInto;

fn main() -> io::Result<()> {
    // 1. Open your input file
    let mut input = File::open("output.txt")?;
    
    // 2. Create an output file for the results
    let mut output = File::create("output2.txt")?;

    // 3. Define the block size for testing (change this to whatever you need)
    let block_size_bits: usize = 8; 

    let mut buffer =  Vec::new();
    let mut word = Vec::new();

    input.read_to_end(&mut buffer)?; //read the file input into the buffer vector (read bytes not bits, miss the conversion)

    let mut bits_info: Vec<u8> = Vec::new();
    // Iteramos PRIMERO por los bytes, y LUEGO extraemos sus 8 bits
    for byte in &buffer {
        for b in 0..8 {
            bits_info.push((byte >> b) & 1);
        }
    }

    let control_bits_quantity = block_size_bits.trailing_zeros() as usize;

    for i in (0..(bits_info.len())).step_by(block_size_bits) {
        let mut bits_info_internal = vec![0; block_size_bits];
        let mut step_buffer = i;

        // Llenar el bloque actual
        for j in 0..block_size_bits {
            if step_buffer < bits_info.len() {
                bits_info_internal[j] = bits_info[step_buffer];
                step_buffer += 1;
            }
        }

        let mut syndrome = 0;
        
        // Calcular el síndrome
        for j in 0..control_bits_quantity {
            let parity_position = 1 << j;
            let mut parity_value = 0;

            // Iterar SOLO hasta block_size_bits - 1
            for bit_position in 1..(block_size_bits - 1) {
                if (bit_position & parity_position) != 0 {
                    parity_value ^= bits_info_internal[bit_position];
                }
            }

            if parity_value != 0 {
                syndrome += parity_position;
            }
        }

        // Calcular la paridad global de todo el bloque
        let mut overall_parity = 0;
        for bit in &bits_info_internal {
            overall_parity ^= bit;
        }

        // Corrección de errores
        if syndrome != 0 {
            // Voltear el bit defectuoso usando XOR
            bits_info_internal[syndrome] ^= 1;
        } else if overall_parity != 0 {
             // Error solo en el bit de paridad global final
             bits_info_internal[block_size_bits - 1] ^= 1;
        }

        // Extraer SOLO los bits de información
        for j in 1..(block_size_bits - 1) {
            // Un truco de bits: si (j & (j - 1)) != 0, entonces 'j' NO es potencia de 2
            if (j & (j - 1)) != 0 {
                word.push(bits_info_internal[j]);
            }
        }
    }

    let mut output_bytes: Vec<u8> = Vec::new();
    
    // Agrupamos de a 8 bits
    for chunk in word.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << i; // Reconstruimos el byte
        }
        output_bytes.push(byte);
    }
    
    println!("Total de bits recuperados en 'word': {}", word.len());
    println!("Total de bytes a escribir: {}", output_bytes.len());
    // 3. Escribir los bytes empaquetados en el archivo de salida
    output.write_all(&output_bytes)?;
    output.flush()?; // Asegura la escritura física
    println!("Archivo escrito correctamente.");    

    Ok(())
}
