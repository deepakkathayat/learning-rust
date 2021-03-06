fn check_question(message: &str) -> bool {
    message.trim().ends_with("?")
}

fn check_silence(message: &str) -> bool {
    message.trim().is_empty()
}

fn check_yell(message: &str) -> bool {
    message.chars().any(char::is_alphabetic) && !message.chars().any(char::is_lowercase)
}

pub fn reply(message: &str) -> &str {
    match (
        check_question(message),
        check_yell(message),
        check_silence(message),
    ) {
        (_, _, true) => return "Fine. Be that way!",
        (true, true, _) => return "Calm down, I know what I'm doing!",
        (true, false, _) => return "Sure.",
        (false, true, _) => return "Whoa, chill out!",
        (_, _, _) => return "Whatever.",
    }
}
