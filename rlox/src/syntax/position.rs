#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub length: usize,
    pub offset: usize
}

impl Position {
    pub fn new(length: usize, offset: usize) -> Position {
        Position { length, offset }
    }
}
