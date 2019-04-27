pub fn factors(m: u64) -> Vec<u64> {
    if m <= 1 {
        vec![]
    } else {
        let mut n: u64 = m;
        let mut factor: u64 = 2;
        let mut result: Vec<u64> = Vec::new();
        loop {
            if n % factor == 0 {
                result.push(factor);
                n = n / factor;
            } else {
                factor = factor + 1;
            }
            if n <= 1 {
                break;
            }
        }
        result
    }
}
