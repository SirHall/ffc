#[derive(Debug, Default, Clone)]
pub struct Pos
{
    pub x : usize,
    pub y : usize,
}

impl Pos
{
    pub fn new(x : usize, y : usize) -> Self
    {
        Pos {
            x,
            y,
        }
    }

    // Get back a position relative to our point
    pub fn rel(&self, dx : isize, dy : isize) -> Pos
    {
        Pos {
            x : ((self.x as isize) + dx) as usize,
            y : ((self.y as isize) + dy) as usize,
        }
    }
}
