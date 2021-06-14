use std::fmt;
use std::usize;

static OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Cell {
    fn from_char(c: &char) -> Cell {
        match c {
            '.' => Cell::Floor,
            'L' => Cell::EmptySeat,
            '#' => Cell::OccupiedSeat,
            _ => panic!("Invalid Char {}", c),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Cell::Floor => '.',
                Cell::EmptySeat => 'L',
                Cell::OccupiedSeat => '#',
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<Cell>>);

impl Board {
    fn from_str(input: &str) -> Board {
        let mut ret = vec![];

        for row in input.lines() {
            let mut line: Vec<Cell> = vec![];
            for c in row.chars() {
                line.push(Cell::from_char(&c));
            }
            ret.push(line);
        }

        Board(ret)
    }

    fn ray_cast_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut neighbours_found = 0;
        for (dy, dx) in OFFSETS.iter() {
            let mut multiplier = 1;
            loop {
                let new_y = (y as i32) + dy * multiplier;
                let new_x = (x as i32) + dx * multiplier;
                multiplier += 1;
                if new_x < 0 || new_y < 0 {
                    break;
                }
                if let Some(row) = self.0.get(new_y as usize) {
                    match row.get(new_x as usize) {
                        Some(Cell::Floor) => {
                            continue;
                        }
                        Some(Cell::EmptySeat) => {
                            break;
                        }
                        Some(Cell::OccupiedSeat) => {
                            neighbours_found += 1;
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        neighbours_found
    }

    fn cell_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut ret = 0;
        for (dy, dx) in OFFSETS.iter() {
            let new_y = (y as i32) + dy;
            let new_x = (x as i32) + dx;
            if new_x < 0 || new_y < 0 {
                continue;
            }
            if let Some(row) = self.0.get(new_y as usize) {
                if let Some(Cell::OccupiedSeat) = row.get(new_x as usize) {
                    ret += 1;
                }
            }
            // if let Some(Cell::OccupiedSeat) = self
            //     .0
            //     .get(new_y as usize)
            //     .map(|row| row.get(new_x as usize))
            //     .flatten()
            // {
            //     ret += 1;
            // }
        }
        ret
    }

    fn next(&self, neighbours_limit: u8, use_ray_cast: bool) -> (Board, bool) {
        let mut ret = vec![];
        let mut changes = false;
        for (y, row) in self.0.iter().enumerate() {
            let mut new_row: Vec<Cell> = vec![];
            for (x, cell) in row.iter().enumerate() {
                let neighbors;
                if use_ray_cast {
                    neighbors = self.ray_cast_neighbours(x, y);
                } else {
                    neighbors = self.cell_neighbors(x, y);
                }

                match cell {
                    Cell::Floor => {
                        new_row.push(Cell::Floor);
                    }
                    Cell::EmptySeat => {
                        if neighbors == 0 {
                            new_row.push(Cell::OccupiedSeat);
                            changes = true;
                        } else {
                            new_row.push(Cell::EmptySeat);
                        }
                    }
                    Cell::OccupiedSeat => {
                        if neighbors >= neighbours_limit {
                            new_row.push(Cell::EmptySeat);
                            changes = true;
                        } else {
                            new_row.push(Cell::OccupiedSeat);
                        }
                    }
                }
            }
            ret.push(new_row);
        }

        (Board(ret), changes)
    }

    fn count(&self) -> u32 {
        let mut ret = 0;
        for row in &self.0 {
            for cell in row {
                if let Cell::OccupiedSeat = cell {
                    ret += 1;
                }
            }
        }
        ret
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board: Vec<String> = vec![];

        for line in &self.0 {
            let this_line: String = line.iter().map(Cell::to_string).collect();
            board.push(this_line);
        }

        write!(f, "{}", board.join("\n"))
    }
}

struct BoardIterator {
    board: Board,
    neighbours_limit: usize,
    use_ray_cast: bool,
}

impl Iterator for BoardIterator {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        let (board, is_changed) = self
            .board
            .next(self.neighbours_limit as u8, self.use_ray_cast);
        if !is_changed {
            None
        } else {
            self.board = board.clone();
            Some(board)
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let board = Board::from_str(input);
    let iterator = BoardIterator {
        board,
        neighbours_limit: 4,
        use_ray_cast: false,
    };

    iterator.last().unwrap().count()
}

pub fn part2(input: &str) -> u32 {
    let board = Board::from_str(input);
    let iterator = BoardIterator {
        board,
        neighbours_limit: 5,
        use_ray_cast: true,
    };

    iterator.last().unwrap().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2261);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2039);
    }
}
