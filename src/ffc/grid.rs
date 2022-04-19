use super::pos::Pos;

pub struct Grid<T>
where
    T : PartialEq + Clone,
{
    grid :   Vec<T>,
    width :  usize,
    height : usize,
}

impl<T> Grid<T>
where
    T : PartialEq + Clone,
{
    pub fn new(cells : Vec<T>, width : usize) -> Self
    {
        Self {
            height : width / cells.len(),
            grid : cells,
            width,
        }
    }

    pub fn pos_to_i(&self, pos : Pos) -> usize { pos.y * self.width + pos.x }

    pub fn get(&self, pos : Pos, outer : T) -> T
    {
        if self.is_valid(pos.clone())
        {
            self.grid
                .get(self.pos_to_i(pos))
                .map(|v| v.to_owned())
                .unwrap_or_else(|| (&outer.clone()).to_owned())
        }
        else
        {
            outer.clone()
        }
    }

    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }
    pub fn get_area(&self) -> usize { self.grid.len() }

    pub fn is_valid(&self, pos : Pos) -> bool { pos.x < self.width && pos.y < self.height }

    pub fn compare(
        a : &Grid<T>,
        a_center : Pos,
        b : &Grid<T>,
        b_center : Pos,
        radius : isize,
        unset : T,
        outer : T,
    ) -> bool
    {
        for dx in (-radius)..radius
        {
            for dy in (-radius)..radius
            {
                let a_pos = a_center.rel(dx, dy);
                let b_pos = b_center.rel(dx, dy);

                // unset - A tile that has not yet been given a value
                // outer - A tile that falls outside of the grid

                let a_tile = a.get(a_pos, outer.clone());
                let b_tile = b.get(b_pos, outer.clone());

                // A cool trick is to set 'outer' to 'unset'

                if a_tile != b_tile && a_tile != unset && b_tile != unset
                {
                    return false;
                }
            }
        }

        true
    }
}
