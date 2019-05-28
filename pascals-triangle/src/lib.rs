pub struct PascalsTriangle {
    row_count: u32,
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut P = PascalsTriangle {
            row_count: row_count,
            rows: vec![vec![]],
        };
        P.rows = P.rows();
        P
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut r: u32 = self.row_count;
        let mut v: Vec<Vec<u32>> = Vec::new();
        while r > 0 {
            let row_size = self.row_count - r + 1;
            let mut row: Vec<u32> = Vec::new();

            r -= 1;
        }
        v
    }
}
