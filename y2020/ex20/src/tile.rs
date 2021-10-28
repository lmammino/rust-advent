use std::convert::TryInto;

pub const TOP: usize = 0;
pub const RTOP: usize = 1;
pub const BOTTOM: usize = 2;
pub const RBOTTOM: usize = 3;
pub const LEFT: usize = 4;
pub const RLEFT: usize = 5;
pub const RIGHT: usize = 6;
pub const RRIGHT: usize = 7;

#[derive(Debug, Clone)]
pub struct Tile<const N: usize> {
    pub id: u16,
    pub cells: [[char; N]; N],
    pub borders: [[char; N]; 8],
}

impl<const N: usize> Tile<N> {
    pub fn new(id: u16, cells: [[char; N]; N]) -> Self {
        // In the all following lines the assignment means copy (array implements Copy trait)
        let top = cells[0];
        let mut rtop = top;
        rtop.reverse();
        let bottom = cells[cells.len() - 1];
        let mut rbottom = bottom;
        rbottom.reverse();
        // `[char; N]` doens't implement FromIterator, so `collect` cannot be used to transform a iterator into an array.
        // Instead, it is possible to convert Vec<char> into a fixed length array of char through `try_into` method of the trait `TryInto`.
        let left: [char; N] = cells
            .iter()
            .map(|r| r[0])
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let mut rleft = left;
        rleft.reverse();
        let right: [char; N] = cells
            .iter()
            .map(|r| r[r.len() - 1])
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let mut rright = right;
        rright.reverse();

        let borders: [[char; N]; 8] = [top, rtop, bottom, rbottom, left, rleft, right, rright];

        Tile { id, cells, borders }
    }

    pub fn is_neighbour_of(&self, tile: &Tile<N>) -> bool {
        for border in &self.borders {
            if tile.borders.contains(border) {
                return true;
            }
        }

        false
    }

    // rotating 90 deg clockwise
    pub fn rotate(&self) -> Self {
        let rotated_cells: [[char; N]; N] = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut col: [char; N] = [' '; N];
                for j in 0..N {
                    col[N - 1 - j] = self.cells[j][i];
                }
                col
            })
            .collect::<Vec<[char; N]>>()
            .try_into()
            .unwrap();

        Tile::new(self.id, rotated_cells)
    }

    pub fn flip_horiz(&self) -> Self {
        let flipped_cells: [[char; N]; N] = self
            .cells
            .iter()
            .map(|row| {
                let mut rev_row = *row;
                rev_row.reverse();
                rev_row
            })
            .collect::<Vec<[char; N]>>()
            .try_into()
            .unwrap();

        Tile::new(self.id, flipped_cells)
    }

    pub fn flip_vert(&self) -> Self {
        let mut flipped_cells = self.cells;
        flipped_cells.reverse();

        Tile::new(self.id, flipped_cells)
    }
}

pub fn fit_tile_bottom<const N: usize>(left: &Tile<N>, right: &Tile<N>) -> Option<Tile<N>> {
    let tile_overlapping_border = right
        .borders
        .iter()
        .position(|x| *x == left.borders[BOTTOM])?;

    Some(match tile_overlapping_border {
        TOP => right.clone(),
        BOTTOM => right.flip_vert(),
        RTOP => right.flip_horiz(),
        RBOTTOM => right.flip_horiz().flip_vert(),
        RRIGHT => right.rotate().flip_horiz(),
        RIGHT=> right.rotate().rotate().rotate(),
        RLEFT => right.rotate(),
        LEFT => right.rotate().flip_vert(),
        _ => unreachable!(),
    })
}

pub fn fit_tile_right<const N: usize>(left: &Tile<N>, right: &Tile<N>) -> Option<Tile<N>> {
    let right_tile_overlapping_border = right
        .borders
        .iter()
        .position(|x| *x == left.borders[RIGHT])?;

    Some(match right_tile_overlapping_border {
        LEFT => right.clone(),
        RLEFT => right.flip_vert(),
        RIGHT => right.flip_horiz(),
        RRIGHT => right.flip_horiz().flip_vert(),
        TOP => right.rotate().flip_horiz(),
        RTOP => right.rotate().rotate().rotate(),
        BOTTOM => right.rotate(),
        RBOTTOM => right.rotate().flip_vert(),
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_tile() {
        let cells = [
            ['a', 'b', 'c', 'd'],
            ['e', 'f', 'g', 'h'],
            ['i', 'j', 'k', 'l'],
            ['m', 'n', 'o', 'p'],
        ];

        let tile = Tile::new(0, cells);

        let rotated_tile = tile.rotate();

        let expected = [
            ['m', 'i', 'e', 'a'],
            ['n', 'j', 'f', 'b'],
            ['o', 'k', 'g', 'c'],
            ['p', 'l', 'h', 'd'],
        ];

        assert_eq!(rotated_tile.cells, expected)
    }

    #[test]
    fn flip_horiz_tile() {
        let cells = [
            ['a', 'b', 'c', 'd'],
            ['e', 'f', 'g', 'h'],
            ['i', 'j', 'k', 'l'],
            ['m', 'n', 'o', 'p'],
        ];

        let tile = Tile::new(0, cells);

        let flipped_tile = tile.flip_horiz();

        let expected = [
            ['d', 'c', 'b', 'a'],
            ['h', 'g', 'f', 'e'],
            ['l', 'k', 'j', 'i'],
            ['p', 'o', 'n', 'm'],
        ];

        assert_eq!(flipped_tile.cells, expected)
    }

    #[test]
    fn flip_vert_tile() {
        let cells = [
            ['a', 'b', 'c', 'd'],
            ['e', 'f', 'g', 'h'],
            ['i', 'j', 'k', 'l'],
            ['m', 'n', 'o', 'p'],
        ];

        let tile = Tile::new(0, cells);

        let flipped_tile = tile.flip_vert();

        let expected = [
            ['m', 'n', 'o', 'p'],
            ['i', 'j', 'k', 'l'],
            ['e', 'f', 'g', 'h'],
            ['a', 'b', 'c', 'd'],
        ];

        assert_eq!(flipped_tile.cells, expected)
    }

    #[test]
    fn test_fit_tile_right() {
        let left = [
            ['l', 'l', 'l', '1'],
            ['l', 'l', 'l', '2'],
            ['l', 'l', 'l', '3'],
            ['l', 'l', 'l', '4'],
        ];
        let left_tile = Tile::new(0, left);

        let test_data = [
            // LEFT
            [
                ['1', 'x', 'x', 'x'],
                ['2', 'x', 'x', 'x'],
                ['3', 'x', 'x', 'x'],
                ['4', 'x', 'x', 'x'],
            ],
            // RLEFT
            [
                ['4', 'x', 'x', 'x'],
                ['3', 'x', 'x', 'x'],
                ['2', 'x', 'x', 'x'],
                ['1', 'x', 'x', 'x'],
            ],
            // RIGHT
            [
                ['x', 'x', 'x', '1'],
                ['x', 'x', 'x', '2'],
                ['x', 'x', 'x', '3'],
                ['x', 'x', 'x', '4'],
            ],
            // RRIGHT
            [
                ['x', 'x', 'x', '4'],
                ['x', 'x', 'x', '3'],
                ['x', 'x', 'x', '2'],
                ['x', 'x', 'x', '1'],
            ],
            // TOP
            [
                ['1', '2', '3', '4'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
            ],
            // RTOP
            [
                ['4', '3', '2', '1'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
            ],
            // BOTTOM
            [
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['1', '2', '3', '4'],
            ],
            // RBOTTOM
            [
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['4', '3', '2', '1'],
            ],
        ];

        let expected = [
            ['1', 'x', 'x', 'x'],
            ['2', 'x', 'x', 'x'],
            ['3', 'x', 'x', 'x'],
            ['4', 'x', 'x', 'x'],
        ];

        for right in test_data.iter() {
            let right_tile = Tile::new(1, *right);
            let test_tile = fit_tile_right::<4>(&left_tile, &right_tile).unwrap();
            assert_eq!(test_tile.cells, expected);
        }
    }

    #[test]
    fn test_fit_tile_right_not_matching() {
        let left = [
            ['x', 'x', 'x', 'x'],
            ['x', 'x', 'x', 'x'],
            ['x', 'x', 'x', 'x'],
            ['1', '2', '3', '4'],
        ];
        let left_tile = Tile::new(0, left);

        let right = [
            ['1', 'y', 'y', 'y'],
            ['2', 'y', 'y', 'y'],
            ['3', 'y', 'y', 'y'],
            ['4', 'y', 'y', 'y'],
        ];
        let right_tile = Tile::new(0, right);

        // these two tiles cannot fit, so we expect a None
        let result = fit_tile_right::<4>(&left_tile, &right_tile);

        assert!(result.is_none());
    }
}
