pub struct PascalsTriangle {
    row_count: u32,
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut p_triangle = PascalsTriangle {
            row_count: row_count,
            rows: vec![vec![]],
        };
        p_triangle.rows = p_triangle.rows();
        p_triangle
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut r: u32 = self.row_count;
        let mut v: Vec<Vec<u32>> = Vec::new();
        while r > 0 {
            let row_size: usize = (self.row_count - r + 1) as usize;
            let mut row: Vec<u32> = Vec::new();

            for i in 0..row_size {
                if i == 0 || i == row_size - 1 {
                    row.push(1);
                } else {
                    row.push(v[row_size - 2][i - 1] + v[row_size - 2][i]);
                }
            }
            v.push(row);
            r -= 1;
        }
        v
    }
}
