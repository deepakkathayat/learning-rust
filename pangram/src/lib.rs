/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let pangram: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut sorted_sentence: Vec<char> = sentence.trim().chars().collect();
    sorted_sentence.sort();
    println("{:?}", sorted_sentence);
    pangram == sorted_sentence
}
