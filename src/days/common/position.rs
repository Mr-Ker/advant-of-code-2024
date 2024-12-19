
#[derive(Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new() -> Position {
        Position{x: 0, y: 0}
    }
}