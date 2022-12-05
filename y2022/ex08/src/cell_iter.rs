pub(crate) struct CellIterator<'a, T> {
    grid: &'a Vec<Vec<T>>,
    row: usize,
    max_row: usize,
    col: usize,
    max_col: usize,
}

impl<'a, T> CellIterator<'a, T> {
    pub fn new(grid: &'a Vec<Vec<T>>) -> Self {
        Self {
            grid,
            row: 0,
            max_row: grid.len(),
            col: 0,
            max_col: grid.get(0).map(|row| row.len()).unwrap_or(0),
        }
    }
}

impl<'a, T> Iterator for CellIterator<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.max_row {
            return None;
        }

        let cell = &self.grid[self.row][self.col];
        let col = self.col;
        let row = self.row;

        self.col += 1;
        if self.col >= self.max_col {
            self.col = 0;
            self.row += 1;
        }

        Some((row, col, cell))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result: Vec<(usize, usize, &i32)> = CellIterator::new(&grid).collect();
        assert_eq!(
            result,
            vec![
                (0, 0, &1),
                (0, 1, &2),
                (0, 2, &3),
                (1, 0, &4),
                (1, 1, &5),
                (1, 2, &6)
            ]
        );
    }
}
