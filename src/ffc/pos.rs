#[derive(Debug, Default, Clone)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }

    // Get back a position relative to our point
    pub fn rel(&self, dx: isize, dy: isize) -> Pos {
        Pos {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}
