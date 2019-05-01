pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    for (r, row) in input.iter().enumerate() {
        for (c, item) in row.iter().enumerate() {
            if row.iter().all(|x| x <= item) && (0..input.len()).all(|x| input[x][c] >= *item) {
                result.push((r, c));
            }
        }
    }
    result
}
