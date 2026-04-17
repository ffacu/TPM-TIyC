use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use crate::file_editor;

#[allow(dead_code)]
pub fn call_hamming(opt: isize,path: &str) -> io::Result<()> {
    // Open the target text file in read-only binary mode
    let mut input_file = File::open(path)?;
    
    match opt {
        1 => {
            // Create the destination files for the protected outputs
            let mut out_ha1 = File::create(format!("{}.HA1", path))?;
            // Process 8-bit blocks
            file_editor::hamming_encoding(8, &mut input_file, &mut out_ha1)?;
            // Rewind the input file cursor back to the beginning for the next module
            input_file.seek(SeekFrom::Start(0))?;
        }
        2 => {
            // Create the destination files for the protected outputs
            let mut out_ha2 = File::create(format!("{}.HA2", path))?;
            // Process 1024-bit blocks
            file_editor::hamming_encoding(1024, &mut input_file, &mut out_ha2)?;
            // Rewind the input file cursor back to the beginning for the next module
            input_file.seek(SeekFrom::Start(0))?;
        }
        3 => {
            // Create the destination files for the protected outputs
            let mut out_ha3 = File::create(format!("{}.HA3", path))?;
            // Process 16384-bit blocks
            file_editor::hamming_encoding(16384, &mut input_file, &mut out_ha3)?;
            // Rewind the input file cursor back to the beginning for the next module
            input_file.seek(SeekFrom::Start(0))?;
            
        }
        _ => {
        // The catch-all for any other number
        println!("Invalid option selected. Please choose 1, 2, or 3.");
        // If your function returns a Result, you might want to return an Err here instead!
    }
    }

    Ok(())

}