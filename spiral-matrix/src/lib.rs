pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![vec![]];
    }
    let mut res: Vec<Vec<u32>> = (0..size).map(|_| vec![0; size as usize]).collect();

    let mut i = 0;
    let mut j = 0;
    let mut item = 1;
    loop {
        res[i][j] = item;
        advance(res, i, j);
    }
    res
}
