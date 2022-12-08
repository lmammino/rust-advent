#[derive(Debug, PartialEq)]
pub(crate) enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn step(&self) -> isize {
        match self {
            Dir::Up => -1,
            Dir::Down => 1,
            Dir::Left => -1,
            Dir::Right => 1,
        }
    }
}

pub(crate) struct DirIterator<'a, T> {
    grid: &'a Vec<Vec<T>>,
    direction: Dir,
    curr_col: usize,
    curr_row: usize,
    col_len: usize,
    row_len: usize,
    done: bool,
}

impl<'a, T> DirIterator<'a, T> {
    pub fn new(grid: &'a Vec<Vec<T>>, from_row: usize, from_col: usize, direction: Dir) -> Self {
        Self {
            grid,
            direction,
            curr_row: from_row,
            curr_col: from_col,
            row_len: grid.len(),
            col_len: grid.get(0).map(|row| row.len()).unwrap_or(0),
            done: false,
        }
    }
}

impl<'a, T> Iterator for DirIterator<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done || self.curr_row >= self.row_len || self.curr_col >= self.col_len {
            return None;
        }

        let row = self.curr_row;
        let col = self.curr_col;
        let cell = &self.grid[row][col];

        if (self.direction == Dir::Up && self.curr_row == 0)
            || (self.direction == Dir::Left && self.curr_col == 0)
        {
            self.done = true;
        } else {
            match self.direction {
                Dir::Up | Dir::Down => {
                    self.curr_row = (self.curr_row as isize + self.direction.step()) as usize
                }
                Dir::Left | Dir::Right => {
                    self.curr_col = (self.curr_col as isize + self.direction.step()) as usize
                }
            }
        }

        Some((row, col, cell))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_down() {
        let grid = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let result: Vec<(usize, usize, &i32)> = DirIterator::new(&grid, 0, 1, Dir::Down).collect();
        assert_eq!(result, vec![(0, 1, &1), (1, 1, &5), (2, 1, &9)]);
    }

    #[test]
    fn test_left_right() {
        let grid = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let result: Vec<(usize, usize, &i32)> = DirIterator::new(&grid, 1, 2, Dir::Right).collect();
        assert_eq!(result, vec![(1, 2, &6), (1, 3, &7)]);
    }

    #[test]
    fn test_down_top() {
        let grid = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let result: Vec<(usize, usize, &i32)> = DirIterator::new(&grid, 2, 3, Dir::Up).collect();
        assert_eq!(result, vec![(2, 3, &11), (1, 3, &7), (0, 3, &3)]);
    }

    #[test]
    fn test_right_left() {
        let grid = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let result: Vec<(usize, usize, &i32)> = DirIterator::new(&grid, 0, 3, Dir::Left).collect();
        assert_eq!(result, vec![(0, 3, &3), (0, 2, &2), (0, 1, &1), (0, 0, &0)]);
    }
}
