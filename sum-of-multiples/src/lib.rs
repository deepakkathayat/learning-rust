fn multiples(limit: u32, x: u32) -> u32 {
    if limit < x || x == 0 {
        0
    } else if limit % x == 0 {
        (1..limit / x).map(|i| i * x).sum()
    } else {
        (1..(limit / x) + 1).map(|i| i * x).sum()
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let list = factors.to_vec();

    list.into_iter().map(|x| multiples(limit, x)).sum()
}
