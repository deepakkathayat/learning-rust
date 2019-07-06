pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    if phrase.len() == 0 {
        return result;
    }

    let s = phrase.as_bytes();
    result.push(s[0] as char);
    for (i, &ch) in s.iter().enumerate() {
        if ch == b' ' && i + 1 != phrase.len() {
            result.push(s[i + 1] as char);
        } else if ch == b'-' && i + 1 != phrase.len() {
            result.push(s[i + 1] as char);
        } else if ch.is_ascii_uppercase() {
            result.push(ch as char);
        }
    }

    result.to_uppercase()
}
