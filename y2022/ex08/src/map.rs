use crate::{cell_iter::CellIterator, dir_iter::Dir, dir_iter::DirIterator};
use std::{ops::Deref, str::FromStr};

#[derive(Debug)]
pub(crate) struct Map {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Ok(Map {
            height: grid.len(),
            width: grid.get(0).map(|l| l.len()).unwrap_or(0),
            grid,
        })
    }
}

impl Deref for Map {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl Map {
    pub fn iter_cells(&self) -> CellIterator<'_, u8> {
        CellIterator::new(&self.grid)
    }

    pub fn iters_from_cell(&self, row: usize, col: usize) -> [DirIterator<'_, u8>; 4] {
        [
            DirIterator::new(&self.grid, row, col, Dir::Up),
            DirIterator::new(&self.grid, row, col, Dir::Right),
            DirIterator::new(&self.grid, row, col, Dir::Down),
            DirIterator::new(&self.grid, row, col, Dir::Left),
        ]
    }

    pub fn all_outer_lines_of_sight(&self) -> Vec<DirIterator<'_, u8>> {
        let mut res = Vec::new();
        // top-down
        for col in 0..self.height {
            res.push(DirIterator::new(&self.grid, 0, col, Dir::Down));
        }

        // right-left
        for row in 0..self.width {
            res.push(DirIterator::new(&self.grid, row, self.width - 1, Dir::Left));
        }

        // down-up
        for col in (0..self.width).rev() {
            res.push(DirIterator::new(&self.grid, self.height - 1, col, Dir::Up));
        }

        // left-right
        for row in (0..self.height).rev() {
            res.push(DirIterator::new(&self.grid, row, 0, Dir::Right));
        }

        res
    }
}
