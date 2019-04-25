use itertools::Itertools;

fn multiples(limit: u32, x: u32) -> Vec<u32> {
    if limit < x || x == 0 {
        vec![0]
    } else if limit % x == 0 {
        (1..limit / x).map(|i| i * x).collect::<Vec<u32>>()
    } else {
        (1..(limit / x) + 1).map(|i| i * x).collect::<Vec<u32>>()
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let list = factors.to_vec();

    list.into_iter()
        .map(|x| multiples(limit, x))
        .flatten()
        .unique()
        .sum()
}
