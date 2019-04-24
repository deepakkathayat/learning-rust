use std::iter;

pub fn build_proverb(list: &[&str]) -> String {
    let list_vec: Vec<_> = list.to_vec();

    if list_vec.is_empty() {
        String::new()
    } else {
        let last_line = format!("And all for the want of a {}.", list_vec[0]);
        list_vec
            .windows(2)
            .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
            .chain(iter::once(last_line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
