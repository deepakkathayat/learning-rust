pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    if phrase.len() == 0 {
        return result;
    }

    result.push(phrase[0]);
    for (i, &ch) in phrase.iter().enumerate() {
        if ch == ' ' && i + 1 != phrase.len() {
            result.push(phrase[i + 1]);
        }
    }
    result
}
