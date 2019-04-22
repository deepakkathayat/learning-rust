fn check_question(message: &String) -> bool {
    if message.ends_with("?") {
        true
    } else {
        false
    }
}

fn check_silence(message: &String) -> bool {
    if message.replace(" ", "") == "" {
        true
    } else {
        false
    }
}

fn check_yell(message: &String) -> bool {
    if message.to_ascii_uppercase() == *message {
        true
    } else {
        false
    }
}

pub fn reply(message: &str) -> &str {
    let s = message.to_string();

    match (check_question(&s), check_yell(&s), check_silence(&s)) {
        (true, true, _) => return "Calm down, I know what I'm doing!",
        (true, false, _) => return "Sure.",
        (false, true, _) => return "Whoa, chill out!",
        (_, _, true) => return "Fine. Be that way!",
        (_, _, _) => return "Whatever.",
    }
}
