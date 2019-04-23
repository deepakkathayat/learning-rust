fn bottles(n: i32) -> &'static str {
    match n {
        1 => "1 bottle of beer",
        0 => "No more bottles of beer",
        -1 => "99 bottles of beer",
        _ => (n.to_string() + " bottles of beer").as_str(),
    }
}

fn phrase(n: i32) -> &'static str {
    match n {
        1 => "Take it down and pass it around,",
        0 => "Go to the store and buy some more,",
        _ => "Take one down and pass it around,",
    }
}

pub fn verse(n: i32) -> String {
    let n_bottles = bottles(n);
    let first_sentence = n_bottles.to_string() + " on the wall, " + &n_bottles + ".";

    let n_phrase = phrase(n);
    let nminus_bottles = bottles(n - 1);
    let second_sentence = n_phrase.to_string() + " " + &nminus_bottles + " on the wall.";

    first_sentence + "\n" + &second_sentence
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result = String::new();

    let mut n: i32 = start;
    while n > end {
        result = result + &verse(n) + "\n\n";
        n -= 1;
    }
    result += &verse(n);

    result
}
