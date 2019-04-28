pub fn brackets_are_balanced(string: &str) -> bool {
    let mut chars: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => chars.push(c),
            '}' => {
                if !chars.ends_with(&['{']) {
                    return false;
                }
                chars.pop();
            }
            ']' => {
                if !chars.ends_with(&['[']) {
                    return false;
                }
                chars.pop();
            }
            ')' => {
                if !chars.ends_with(&['(']) {
                    return false;
                }
                chars.pop();
            }
            _ => {}
        }
    }
    chars.is_empty()
}
