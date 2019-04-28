pub fn brackets_are_balanced(string: &str) -> bool {
    let mut chars: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => chars.push(c),
            '}' => {
                if chars.pop().unwrap() != '{' {
                    return false;
                }
            }
            ']' => {
                if chars.pop().unwrap() != '[' {
                    return false;
                }
            }
            ')' => {
                if chars.pop().unwrap() != '(' {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}
