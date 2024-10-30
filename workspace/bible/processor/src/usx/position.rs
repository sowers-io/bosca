#[derive(Clone)]
pub struct Position {
    pub start: i64,
    pub end: i64,
}

impl Position {
    pub fn new(start: i64) -> Self {
        Self { start, end: start }
    }
}
