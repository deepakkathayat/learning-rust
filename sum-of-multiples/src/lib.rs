fn multiples(x: u32, limit: u32) -> u32 {
    (1..limit / x).map(|i| i * x).filter(|n| n < &limit).sum()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let list = factors.to_vec();

    list.into_iter().map(|x| multiples(x, limit)).sum()
}
