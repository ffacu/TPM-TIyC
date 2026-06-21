use rayon::prelude::*;
use std::collections::HashMap;

pub fn char_frequencies(lines: &Vec<String>) -> HashMap<char, u64> {
    // Use Rayon to parallelize the frequency counting across lines. Each line is processed in parallel, and the results are combined at the end.
    lines
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut freqs: HashMap<_, _>, line: &String| {
                for ch in line.chars() {
                    *freqs.entry(ch).or_insert(0) += 1;
                }
                freqs
            },
        )
        .reduce(
            || HashMap::new(),
            |mut freqs1, freqs2| {
                freqs2
                    .into_iter()
                    .for_each(|(ch, n)| *freqs1.entry(ch).or_insert(0) += n);
                freqs1
            },
        )
        //returns a HashMap with the frequency of each character in the input lines
}

pub fn word_frequencies(lines: &Vec<String>) -> HashMap<String, u64> {
    lines
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut freqs: HashMap<_, _>, line: &String| {
                for word in line.split(' ') {
                    *freqs.entry(word.to_string()).or_insert(0) += 1;
                }
                freqs
            },
        )
        .reduce(
            || HashMap::new(),
            |mut freqs1, freqs2| {
                freqs2
                    .into_iter()
                    .for_each(|(word, n)| *freqs1.entry(word).or_insert(0) += n);
                freqs1
            },
        )
        //returns a HashMap with the frequency of each word in the input lines
}
