pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    if phrase.len() == 0 {
        return result;
    }

    let s = phrase.as_bytes();

    for (i, &ch) in s.iter().enumerate() {
        if i == 0 {
            result.push(ch as char);
        } else if s[i - 1] == b' ' || s[i - 1] == b'-' {
            result.push(ch as char);
        } else if ch.is_ascii_uppercase() && s[i - 1].is_ascii_lowercase() {
            result.push(ch as char);
        }
    }

    result.to_uppercase()
}
