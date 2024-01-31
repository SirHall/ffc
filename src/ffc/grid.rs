use super::pos::Pos;
use std::hash::Hash;

// TODO: Switch back to this when we have trait aliases
// pub trait GridCellT = PartialEq + Eq + Hash + Clone + Display + Sync + Send;

#[derive(Debug, Clone)]
pub struct Grid<T: PartialEq + Eq + Hash + Clone + Sync + Send> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: PartialEq + Eq + Hash + Clone + Sync + Send> Grid<T> {
    pub fn new(cells: Vec<T>, width: usize) -> Self {
        Self {
            width,
            height: cells.len() / width,
            grid: cells,
        }
    }

    pub fn pos_to_i(&self, pos: &Pos) -> usize {
        ((pos.y * (self.width as isize)) + pos.x) as usize
    }

    pub fn i_to_pos(&self, i: usize) -> Pos {
        Pos::new((i % self.width) as isize, (i / self.width) as isize)
    }

    pub fn get(&self, pos: &Pos, outer: T) -> T {
        if self.is_valid(pos) {
            self.grid
                .get(self.pos_to_i(pos))
                .map(|v| v.to_owned())
                .unwrap_or_else(|| (&outer.clone()).to_owned())
        } else {
            outer
        }
    }

    pub fn set(&mut self, pos: &Pos, val: T) {
        if self.is_valid(pos) {
            let i = self.pos_to_i(pos);
            self.grid[i] = val;
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_area(&self) -> usize {
        self.grid.len()
    }

    pub fn is_valid(&self, pos: &Pos) -> bool {
        pos.x < (self.width as isize) && pos.x >= 0 && pos.y < (self.height as isize) && pos.y >= 0
    }

    pub fn compare(
        a: &Grid<T>,
        a_center: &Pos,
        b: &Grid<T>,
        b_center: &Pos,
        radius: isize,
        unset: T,
        outer: T,
    ) -> bool {
        for dx in (-radius)..=radius {
            for dy in (-radius)..=radius {
                let a_pos = a_center.rel(dx, dy);
                let b_pos = b_center.rel(dx, dy);

                // unset - A tile that has not yet been given a value
                // outer - A tile that falls outside of the grid

                let a_tile = a.get(&a_pos, outer.clone());
                let b_tile = b.get(&b_pos, outer.clone());

                // A cool trick is to set 'outer' to 'unset' so that anything outside the image will match to everything

                if a_tile != b_tile && a_tile != unset && b_tile != unset {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_cells(&self) -> &Vec<T> {
        &self.grid
    }
}
