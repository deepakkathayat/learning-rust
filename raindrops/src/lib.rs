pub fn raindrops(n: u32) -> String {
    let is_divisible_by = |div| n % div == 0;
    let str_divisible_by = |s, div| if is_divisible_by(div) { s } else { "" };

    let result = String::new()
        + str_divisible_by("Pling", 3)
        + str_divisible_by("Plang", 5)
        + str_divisible_by("Plong", 7);

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
