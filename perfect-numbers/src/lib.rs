#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let sum: u64 = (1..num / 2 + 1).into_iter().filter(|&x| num % x == 0).sum();

    match sum {
        sum if sum == num => Ok(Classification::Perfect),
        sum if sum > num => Ok(Classification::Abundant),
        sum if sum < num => Ok(Classification::Deficient),
        _ => None,
    }
}
