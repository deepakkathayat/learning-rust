use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let pangram: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let candidate = sentence
        .chars()
        .filter(|&x| x.is_ascii_alphabetic())
        .map(|x| x.to_ascii_lowercase())
        .collect::<HashSet<char>>();
    pangram == candidate
}
