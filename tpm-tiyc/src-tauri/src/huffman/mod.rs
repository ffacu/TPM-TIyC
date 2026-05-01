pub mod compression;
pub mod freqs;
pub mod huffman;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Words,
    Chars,
}

pub fn compress_file(input: PathBuf, output: PathBuf, mode: Mode) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(&input)?;
    let lines: Vec<_> = text.split('\n').map(|x| x.to_string()).collect();

    let compressed = match mode {
        Mode::Words => compression::compress(&lines, freqs::word_frequencies, |line| {
            line.split_ascii_whitespace().map(|token| token.to_string())
        }),
        Mode::Chars => {
            compression::compress(&lines, freqs::char_frequencies, |line| line.chars())
        }
    }?;

    let mut out_f = File::create(&output)?;
    out_f.write_all(&compressed)?;
    Ok(())
}

pub fn extract_file(input: PathBuf, output: PathBuf, mode: Mode) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read(&input)?;

    let content = match mode {
        Mode::Words => compression::extract(&data, |tokens: Vec<String>| tokens.join(" "))?,
        Mode::Chars => {
            compression::extract(&data, |tokens: Vec<char>| tokens.into_iter().collect())?
        }
    };

    fs::write(&output, content.join("\n"))?;
    Ok(())
}
