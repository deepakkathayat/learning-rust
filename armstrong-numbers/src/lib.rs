pub fn is_armstrong_number(num: u32) -> bool {
    let mut n: u32 = num;
    let mut digits: Vec<u32> = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n = n / 10;
    }
    let len = digits.len() as u32;
    num == digits.iter().fold(0, |acc, x| acc + x.pow(len))
}
