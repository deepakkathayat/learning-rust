#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let sum = (1..num).iter().filter(|&x| num % x == 0).sum();

    match sum {
        sum if sum == num => Option(Perfect),
        sum if sum < num => Option(Deficient),
        sum if sum > num => Option(Deficient),
    }
}
