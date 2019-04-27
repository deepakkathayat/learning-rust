pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut factor: u64 = 2;
    while n > 1 {
        while n % factor == 0 {
            result.push(factor);
            n = n / factor;
        }
        factor = factor + 1;
    }
    result
}
