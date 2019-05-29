pub struct Triangle {
    x: u64,
    y: u64,
    z: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        match (sides[0], sides[1], sides[2]) {
            (0, _, _) => None,
            (_, 0_, _) => None,
            (_, _, 0) => None,
            (x, y, z) if x + y >= z && y + z >= x && x + z >= y => Some(Triangle { x, y, z }),
            _ => None,
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y && self.y == self.z
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.x == self.y || self.y == self.z || self.x == self.z
    }
}
