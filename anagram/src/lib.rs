use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    fn sort_str(word: &str) -> Vec<char> {
        let mut word: Vec<char> = word.to_lowercase().chars().collect();
        word.sort();
        word
    }
    let word = word.to_lowercase();

    let res: HashSet<&'a str> = possible_anagrams
        .iter()
        .cloned()
        .filter(|s| word != s.to_lowercase() && sort_str(&word) == sort_str(s))
        .collect();

    res
}
