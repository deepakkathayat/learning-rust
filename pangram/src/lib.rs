/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    (b'a'..b'z').all(|c| sentence.to_lowercase().contains(c as char))
}
