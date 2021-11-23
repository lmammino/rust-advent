use std::convert::TryInto;
use std::fmt;

pub const TOP: usize = 0;
pub const RTOP: usize = 1;
pub const BOTTOM: usize = 2;
pub const RBOTTOM: usize = 3;
pub const LEFT: usize = 4;
pub const RLEFT: usize = 5;
pub const RIGHT: usize = 6;
pub const RRIGHT: usize = 7;

/* DRAGON
                  #
#    ##    ##    ###
 #  #  #  #  #  #
*/

pub const DRAGON: [[usize; 2]; 15] = [
    [0, 18],
    [1, 0],
    [1, 5],
    [1, 6],
    [1, 11],
    [1, 12],
    [1, 17],
    [1, 18],
    [1, 19],
    [2, 1],
    [2, 4],
    [2, 7],
    [2, 10],
    [2, 13],
    [2, 16],
];

pub const DRAGON_WIDTH: usize = 20;
pub const DRAGON_HEIGHT: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct Tile<const N: usize> {
    pub id: u16,
    pub cells: [[char; N]; N],
    pub borders: [[char; N]; 8],
}

impl<const N: usize> fmt::Display for Tile<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.cells {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        write!(f, "")
    }
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

    pub fn count_dragons(&self) -> usize {
        if N < DRAGON_WIDTH || N < DRAGON_HEIGHT {
            return 0;
        }

        let mut count = 0;

        for row in 0..(N - DRAGON_HEIGHT + 1) {
            for col in 0..(N - DRAGON_WIDTH + 1) {
                let matches = DRAGON
                    .iter()
                    .all(|p| self.cells[row + p[0]][col + p[1]] == '#');

                if matches {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn count_sharps(&self) -> usize {
        let mut count = 0;
        for row in 0..N {
            for col in 0..N {
                if self.cells[row][col] == '#' {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn fit_tile_bottom<const N: usize>(top: &Tile<N>, bottom: &Tile<N>) -> Option<Tile<N>> {
    let tile_overlapping_border = bottom
        .borders
        .iter()
        .position(|x| *x == top.borders[BOTTOM])?;

    Some(match tile_overlapping_border {
        TOP => *bottom,
        BOTTOM => bottom.flip_vert(),
        RTOP => bottom.flip_horiz(),
        RBOTTOM => bottom.flip_horiz().flip_vert(),
        LEFT => bottom.rotate().flip_horiz(),
        RIGHT => bottom.rotate().rotate().rotate(),
        RLEFT => bottom.rotate(),
        RRIGHT => bottom.rotate().flip_vert(),
        _ => unreachable!(),
    })
}

pub fn fit_tile_right<const N: usize>(left: &Tile<N>, right: &Tile<N>) -> Option<Tile<N>> {
    let right_tile_overlapping_border = right
        .borders
        .iter()
        .position(|x| *x == left.borders[RIGHT])?;

    Some(match right_tile_overlapping_border {
        LEFT => *right,
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
    fn test_fit_tile_bottom() {
        let top = [
            ['t', 't', 't', 't'],
            ['t', 't', 't', 't'],
            ['t', 't', 't', 't'],
            ['1', '2', '3', '4'],
        ];
        let top_tile = Tile::new(0, top);

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
            ['1', '2', '3', '4'],
            ['x', 'x', 'x', 'x'],
            ['x', 'x', 'x', 'x'],
            ['x', 'x', 'x', 'x'],
        ];

        for bottom in test_data.iter() {
            let bottom_tile = Tile::new(1, *bottom);
            let test_tile = fit_tile_bottom::<4>(&top_tile, &bottom_tile).unwrap();
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

    #[test]
    fn test_fit_tile_bottom_not_matching() {
        let top = [
            ['x', 'x', 'x', '1'],
            ['x', 'x', 'x', '2'],
            ['x', 'x', 'x', '3'],
            ['x', 'x', 'x', '4'],
        ];
        let top_tile = Tile::new(0, top);

        let bottom = [
            ['1', 'y', 'y', 'y'],
            ['2', 'y', 'y', 'y'],
            ['3', 'y', 'y', 'y'],
            ['4', 'y', 'y', 'y'],
        ];
        let bottom_tile = Tile::new(0, bottom);

        // these two tiles cannot fit, so we expect a None
        let result = fit_tile_bottom::<4>(&top_tile, &bottom_tile);

        assert!(result.is_none());
    }

    #[test]
    fn test_dragon_is_correct() {
        let expected = r#"                  #
#    ##    ##    ###
 #  #  #  #  #  #"#;

        let mut dragon_chars = [[' '; DRAGON_WIDTH]; DRAGON_HEIGHT];
        for point in DRAGON {
            dragon_chars[point[0]][point[1]] = '#';
        }

        let rendered_dragon = dragon_chars
            .iter()
            .map(|row| {
                row.iter()
                    .cloned()
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(expected, rendered_dragon.as_str());
    }

    #[test]
    fn test_count_dragons_and_sharps() {
        let raw_tile = r#"....................
....................
....................
..................#.
#....##....##....###
.#..#..#..#..#..#...
....................
....................
....................
....................
....................
..................#.
#....##....##....###
.#..#..#..#..#..#...
....................
....................
....................
....................
....................
...................."#;

        let lines = raw_tile.lines();
        let cells: [[char; 20]; 20] = lines
            .map(|l| l.chars().collect::<Vec<char>>().try_into().unwrap())
            .collect::<Vec<[char; 20]>>()
            .try_into()
            .unwrap();

        let tile = Tile::new(1, cells);

        assert_eq!(tile.count_dragons(), 2);
        assert_eq!(tile.count_sharps(), DRAGON.len() * 2);
    }
}
