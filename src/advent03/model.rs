pub struct Path {
    pub(crate) directions: Vec<Direction>
}

#[derive(PartialEq,Debug,Clone)]
pub enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32)
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}