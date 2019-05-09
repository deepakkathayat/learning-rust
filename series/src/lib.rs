pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|x| x.into_iter().collect())
        .collect()
}
