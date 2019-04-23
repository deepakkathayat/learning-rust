pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();

    let mut list_iter = list.iter();

    let mut first_word;

    match list_iter.next() {
        Some(v) => first_word = v.to_string(),
        None => return result,
    }

    let mut word = first_word.clone();
    loop {
        match list_iter.next() {
            Some(x) => {
                result = result + &format!("For want of a {} the {} was lost.\n", word, x);
                word = x.to_string();
            }
            None => {
                if first_word.is_empty() {
                    break;
                } else {
                    result = result + &format!("And all for the want of a {}.", first_word);
                    break;
                }
            }
        }
    }
    result
}
