fn bottles(n: i32) -> String {
    match n {
        1 => "1 bottle of beer".to_string(),
        0 => "no more bottles of beer".to_string(),
        -1 => "99 bottles of beer".to_string(),
        _ => n.to_string() + " bottles of beer",
    }
}

fn first_phrase(n: i32) -> String {
    match n {
        1 => "1 bottle of beer on the wall, 1 bottle of beer.".to_string(),
        0 => "No more bottles of beer on the wall, no more bottles of beer.".to_string(),
        _ => bottles(n) + " on the wall, " + &bottles(n) + ".",
    }
}

fn second_phrase(n: i32) -> String {
    match n {
        1 => "Take it down and pass it around, ".to_string() + &bottles(n - 1) + " on the wall.",
        0 => "Go to the store and buy some more, ".to_string() + &bottles(n - 1) + " on the wall.",
        _ => "Take one down and pass it around, ".to_string() + &bottles(n - 1) + " on the wall.",
    }
}

pub fn verse(n: i32) -> String {
    first_phrase(n) + "\n" + &second_phrase(n) + "\n"
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result = String::new();

    let mut n: i32 = start;
    while n > end {
        result = result + &verse(n) + "\n";
        n -= 1;
    }
    result = result + &verse(n);

    result
}
