use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let mut w = word.chars().collect::<Vec<char>>();
    w.sort();
    let res = possible_anagrams.iter().filter(|s| {let mut st = s.chars().collect::<Vec<char>>(); st.sort(); st} == w);

    let result: HashSet<&'a str>  = HashSet::new();
    for item in res {
        result.insert(*item: &'a str);
    }
    result
}
