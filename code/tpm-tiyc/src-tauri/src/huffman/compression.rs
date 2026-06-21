use bit_vec::BitVec;
use rayon::prelude::*;
use rmp_serde;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

use crate::huffman::huffman::{self, Tree};
use Tree::*;

/*  Serialize and Deserialize the compressed data, which consists of the encoder (the huffman codes for each token) 
 and the data (the concatenation of the huffman codes of the tokens in the file). */
#[derive(Serialize, Deserialize)]
struct CompressedData<T: Eq + Hash> {
    encoder: HashMap<T, BitVec>,
    data: Vec<BitVec>,
    original_extension: String,
}

pub fn compress<'a, T, FreqsF, TokenExtractor, TokensIter>(
    lines: &'a Vec<String>,
    get_freqs: FreqsF,
    line_to_tokens: TokenExtractor,
    original_extension: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    T: Clone + Eq + Hash + Send + Sync + Serialize,     // T can be cloned, compared, hashed, sent and synchronized across threads, and serialized
    FreqsF: Fn(&'a Vec<String>) -> HashMap<T, u64>,    // Defines a freqsF function 
    TokenExtractor: Fn(&'a str) -> TokensIter + Send + Sync,
    TokensIter: Iterator<Item = T>, // TokensIter is an iterator that yields items of type T
{
    let freqs = get_freqs(lines);
    let tree = huffman::huffman_tree(&freqs);
    let encoder = tree.to_encoder(); // Create huffman codification for the tokens.

    // Collects the file data into a single vector of concatenations of the huffman codes of the tokens.
    let data = lines
        .par_iter()
        .map(|line| {
            line_to_tokens(line)
                .map(|token| encoder.get(&token).unwrap().clone())
                .fold(BitVec::new(), |mut vec1, vec2| {
                    vec1.extend(vec2);
                    vec1
                })
        })
        .collect();

    let compressed_data = CompressedData { encoder, data, original_extension }; 
    rmp_serde::encode::to_vec(&compressed_data).map_err(|err| err.into()) // Serialize the compressed data. Where the first part is the encoder (the huffman codes for each token) and the second part is the data (the concatenation of the huffman codes of the tokens in the file).
}

pub fn extract<'a, T, F>(
    data: &'a Vec<u8>,
    tokens_to_line: F,
) -> Result<(Vec<String>, String), Box<dyn std::error::Error>>
where
    T: Clone + Eq + Hash + Send + Sync + Deserialize<'a>,
    F: Fn(Vec<T>) -> String + Send + Sync,
{
    let CompressedData { encoder, data, original_extension }: CompressedData<T> = rmp_serde::decode::from_slice(data)?; //deserialize to get the encoder and the data

    let decoder = encoder_to_decoder(&encoder);
    //Decode the data into lines.
    let lines = data
        .par_iter()
        .map(|line| {
            let mut tokens = vec![];
            let mut candidate = BitVec::new();

            for bit in line {
                candidate.push(bit);

                //If the stream of bits matches a huffman code, get the token
                match decoder.get(&candidate) {
                    Some(token) => {
                        tokens.push(token.clone());

                        candidate = BitVec::new();
                    }
                    None => (),
                }
            }
            tokens_to_line(tokens)
        })
        .collect();

    Ok((lines, original_extension))
}

pub fn compress_bytes(
    data: &[u8],
    original_extension: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut freqs = HashMap::new();
    for &b in data {
        *freqs.entry(b).or_insert(0) += 1;
    }

    let tree = huffman::huffman_tree(&freqs);
    let encoder = tree.to_encoder();

    let bits: BitVec = data
        .iter()
        .flat_map(|b| encoder.get(b).unwrap().clone())
        .collect();

    let compressed_data = CompressedData {
        encoder,
        data: vec![bits],
        original_extension,
    };
    
    rmp_serde::encode::to_vec(&compressed_data).map_err(|err| err.into())
}

pub fn extract_bytes(
    data: &[u8],
) -> Result<(Vec<u8>, String), Box<dyn std::error::Error>> {
    let CompressedData { encoder, data: compressed_bits, original_extension }: CompressedData<u8> =
        rmp_serde::decode::from_slice(data)?;

    let decoder = encoder_to_decoder(&encoder);
    
    let extracted_data = compressed_bits
        .par_iter()
        .flat_map(|bits| {
            let mut tokens = Vec::new();
            let mut candidate = BitVec::new();

            for bit in bits {
                candidate.push(bit);

                match decoder.get(&candidate) {
                    Some(&token) => {
                        tokens.push(token);
                        candidate = BitVec::new();
                    }
                    None => (),
                }
            }
            tokens
        })
        .collect();

    Ok((extracted_data, original_extension))
}

fn encoder_to_decoder<T: Clone>(encoder: &HashMap<T, BitVec>) -> HashMap<BitVec, T> {
    let mut decoder = HashMap::new();
    //Swap prefix to key to the hasmap
    for (token, prefix) in encoder.clone() {
        decoder.insert(prefix, token);
    }
    decoder
}

impl<T: Eq + Clone + Hash> Tree<T> {
    pub fn to_encoder(&self) -> HashMap<T, BitVec> {
        let mut encoder = HashMap::new();

        // Deep-First Search to transverse the tree and build the encoder.
        let mut stack = vec![(self, BitVec::new())];
        while !stack.is_empty() {
            let (node, path) = stack.pop().unwrap();
            match node {
                Leaf { token, .. } => {
                    encoder.insert(token.clone(), path.clone());
                }
                Node { left, right, .. } => {
                    let mut left_path = path.clone();
                    left_path.push(false);
                    stack.push((left, left_path));

                    let mut right_path = path.clone();
                    right_path.push(true);
                    stack.push((right, right_path));
                }
            }
        }

        encoder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::huffman::freqs;

    #[test]
    fn compress_decompress_test() {
        let lines = vec![
            "hey there! nice to meet you.".to_string(),
            "Serde is a framework for serializing and deserializing Rust data structures"
                .to_string(),
        ];

        let data = compress(&lines, freqs::char_frequencies, |line| line.chars(), "txt".to_string()).unwrap();
        let (res_lines, ext) = extract(&data, |x: Vec<char>| x.into_iter().collect()).unwrap();
        assert_eq!(&lines, &res_lines);
        assert_eq!(ext, "txt");

        let data = compress(&lines, freqs::word_frequencies, |line| {
            line.split(' ').map(|token| token.to_string())
        }, "txt".to_string())
        .unwrap();
        let (res_lines, ext) = extract(&data, |x: Vec<String>| x.join(" ")).unwrap();
        assert_eq!(&lines, &res_lines);
        assert_eq!(ext, "txt");
    }
}
