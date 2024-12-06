#[derive(Debug, Clone)]
struct Grid<const T: usize> {
    data: [[char; T]; T],
}

const SEQ_CHARS: [char; 3] = ['M', 'A', 'S'];
const DIRECTIONS_DELTA: [(isize, isize); 8] = [
    (-1, 0),  // top
    (-1, 1),  // top-right
    (0, 1),   // right
    (1, 1),   // down-right
    (1, 0),   // down
    (1, -1),  // down-left
    (0, -1),  // left
    (-1, -1), // top-left
];
const XMAS_POSITIONS: [((isize, isize), (isize, isize)); 2] =
    [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];

impl<const T: usize> Grid<T> {
    fn new(input: &str) -> Self {
        let mut data: [[char; T]; T] = [['.'; T]; T];
        for (line_idx, line) in input.lines().enumerate() {
            for (column_idx, char) in line.chars().enumerate() {
                data[line_idx][column_idx] = char;
            }
        }

        Self { data }
    }

    fn in_bound(pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < T as isize && pos.1 < T as isize
    }

    fn count_matches_at(&self, pos: (usize, usize)) -> usize {
        DIRECTIONS_DELTA
            .iter()
            .filter(|delta| {
                for (char_idx, expected_char) in SEQ_CHARS.iter().enumerate() {
                    let new_pos = (
                        pos.0 as isize + (delta.0 * (char_idx + 1) as isize),
                        pos.1 as isize + (delta.1 * (char_idx + 1) as isize),
                    );
                    if !Self::in_bound(new_pos) {
                        return false;
                    }
                    if self.data[new_pos.0 as usize][new_pos.1 as usize] != *expected_char {
                        return false;
                    }
                }
                true
            })
            .count()
    }

    fn count_matches(&self) -> usize {
        let mut occurrences = 0;
        for (line_idx, line) in self.data.iter().enumerate() {
            for (col_idx, c) in line.iter().enumerate() {
                if *c == 'X' {
                    occurrences += self.count_matches_at((line_idx, col_idx));
                }
            }
        }

        occurrences
    }

    fn is_x_mas_at(&self, pos: (usize, usize)) -> bool {
        let [(pos1, pos2), (pos3, pos4)] = XMAS_POSITIONS;
        let new_pos1 = (pos.0 as isize + pos1.0, pos.1 as isize + pos1.1);
        let new_pos2 = (pos.0 as isize + pos2.0, pos.1 as isize + pos2.1);
        let new_pos3 = (pos.0 as isize + pos3.0, pos.1 as isize + pos3.1);
        let new_pos4 = (pos.0 as isize + pos4.0, pos.1 as isize + pos4.1);
        if !Self::in_bound(new_pos1)
            || !Self::in_bound(new_pos2)
            || !Self::in_bound(new_pos3)
            || !Self::in_bound(new_pos4)
        {
            return false;
        }

        let cell1 = self.data[new_pos1.0 as usize][new_pos1.1 as usize];
        let cell2 = self.data[new_pos2.0 as usize][new_pos2.1 as usize];
        let cell3 = self.data[new_pos3.0 as usize][new_pos3.1 as usize];
        let cell4 = self.data[new_pos4.0 as usize][new_pos4.1 as usize];

        (cell1 == 'M' && cell2 == 'S' || cell1 == 'S' && cell2 == 'M')
            && (cell3 == 'M' && cell4 == 'S' || cell3 == 'S' && cell4 == 'M')
    }

    fn count_x_mas_matches(&self) -> usize {
        let mut occurrences = 0;
        for (line_idx, line) in self.data.iter().enumerate() {
            for (col_idx, c) in line.iter().enumerate() {
                if *c == 'A' {
                    occurrences += if self.is_x_mas_at((line_idx, col_idx)) {
                        1
                    } else {
                        0
                    };
                }
            }
        }

        occurrences
    }
}

pub fn part1<const T: usize>(input: &str) -> usize {
    let grid = Grid::<T>::new(input);
    grid.count_matches()
}

pub fn part2<const T: usize>(input: &str) -> usize {
    let grid = Grid::<T>::new(input);
    grid.count_x_mas_matches()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1::<10>(EXAMPLE_INPUT), 18);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::<140>(INPUT), 2483);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::<140>(INPUT), 1925);
    }
}
