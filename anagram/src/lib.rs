use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut w = word.to_lowercase().chars().collect::<Vec<char>>();
    w.sort();
    println!("{:?}", w);
    let res: HashSet<&'a str> = possible_anagrams
        .iter()
        .filter(
            |s| {
                let mut st = s.to_lowercase().chars().collect::<Vec<char>>();
                st.sort();
                println!("{:?}", st);
                st
            } == w && word.to_lowercase() != s.to_lowercase(),
        )
        .map(|s| *s)
        .collect();

    res
}
