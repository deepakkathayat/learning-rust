fn check_question(message: &String) -> bool {
    if message.trim().ends_with("?") {
        true
    } else {
        false
    }
}

fn check_silence(message: &String) -> bool {
    if message.trim() == "" {
        true
    } else {
        false
    }
}

fn check_yell(message: &String) -> bool {
    if message.to_ascii_uppercase() == *message && message.chars().any(char::is_alphabetic) {
        true
    } else {
        false
    }
}

pub fn reply(message: &str) -> &str {
    let s = message.to_string();

    match (check_question(&s), check_yell(&s), check_silence(&s)) {
        (_, _, true) => return "Fine. Be that way!",
        (true, true, _) => return "Calm down, I know what I'm doing!",
        (true, false, _) => return "Sure.",
        (false, true, _) => return "Whoa, chill out!",
        (_, _, _) => return "Whatever.",
    }
}
