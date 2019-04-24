pub fn square_of_sum(n: u32) -> u32 {
    let mut result = 0;

    for i in 1..n + 1 {
        result += i;
    }
    result * result
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut result = 0;

    for i in 1..n + 1 {
        result += i * i;
    }
    result
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
