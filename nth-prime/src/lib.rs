fn next_prime(primes: &mut Vec<u32>) -> u32 {
    let size = primes.len();
    let mut prime_candidate = primes[size - 1] + 2;

    loop {
        if !primes.iter().any(|x| prime_candidate % x == 0) {
            return prime_candidate;
        }
        prime_candidate += 2;
        /* index = 0;

        while index < size {
            if prime_candidate % primes[index] == 0 {
                break;
            }
            index += 1;
        }
        if index == size {
            return prime_candidate;
        }
         */
    }
}

pub fn nth(n: u32) -> u32 {
    let n = n + 1;
    let mut primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13];

    loop {
        if n > primes.len() as u32 {
            let next_prime_number = next_prime(&mut primes);
            primes.push(next_prime_number);
        } else {
            return primes[(n - 1) as usize];
        }
    }
}
