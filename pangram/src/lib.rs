/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let pangram: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut sorted_sentence = sentence.chars().collect::<Vec<char>>();
    sorted_sentence.sort();
    pangram == sorted_sentence
}
