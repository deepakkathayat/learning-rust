use DIRECTION::*;
enum DIRECTION {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn advance(res: &Vec<Vec<u32>>, i: &usize, j: &usize, dir: &DIRECTION) {}
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![vec![]];
    }
    let mut res: Vec<Vec<u32>> = (0..size).map(|_| vec![0; size as usize]).collect();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut item: u32 = 1;
    let mut dir: DIRECTION = RIGHT;
    loop {
        res[i][j] = item;
        advance(&res, &i, &j, &dir);
        item += 1;
    }
    res
}
