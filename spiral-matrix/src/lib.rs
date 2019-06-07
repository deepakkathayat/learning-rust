use DIRECTION::*;
enum DIRECTION {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

fn advance(res: &Vec<Vec<u32>>, i: &mut usize, j: &mut usize, dir: &mut DIRECTION) {
    match dir {
        RIGHT => {
            if *j + 1 == res.len() {
                *dir = DOWN;
                *i = *i + 1;
            } else if res[*i][*j + 1] != 0 {
                *dir = DOWN;
                *i = *i + 1;
            } else {
                *j = *j + 1;
            }
        }
        DOWN => {
            if *i + 1 == res.len() {
                *dir = LEFT;
                *j = *j - 1;
            } else if res[*i + 1][*j] != 0 {
                *dir = LEFT;
                *j = *j - 1;
            } else {
                *i = *i + 1;
            }
        }
        LEFT => {
            if *j == 0 {
                *dir = UP;
                *i = *i - 1;
            } else if res[*i][*j - 1] != 0 {
                *dir = UP;
                *i = *i - 1;
            } else {
                *j = *j - 1;
            }
        }
        UP => {
            if res[*i - 1][*j] != 0 {
                *dir = RIGHT;
                *j = *j + 1;
            } else {
                *i = *i - 1;
            }
        }
    }
}
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }
    let mut res: Vec<Vec<u32>> = (0..size).map(|_| vec![0; size as usize]).collect();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut item: u32 = 1;
    let mut dir: DIRECTION = RIGHT;
    loop {
        res[i][j] = item;

        if item == size * size {
            break;
        }

        advance(&res, &mut i, &mut j, &mut dir);
        item += 1;
    }
    res
}
