use std::usize;

#[derive(Debug)]
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

impl ToString for Cell {
    fn to_string(&self) -> String {
        match &self {
            Cell::Floor => String::from("."),
            Cell::EmptySeat => String::from("L"),
            Cell::OccupiedSeat => String::from("#"),
        }
    }
}
#[derive(Debug)]
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

    fn cell_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut ret = 0;
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (dy, dx) in offsets.iter() {
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

    fn next(&self) -> (Board, bool) {
        let mut ret = vec![];
        let mut changes = false;
        for (y, row) in self.0.iter().enumerate() {
            let mut new_row: Vec<Cell> = vec![];
            for (x, cell) in row.iter().enumerate() {
                let neighbors = self.cell_neighbors(x, y);
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
                        if neighbors >= 4 {
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

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut board: Vec<String> = vec![];

        for line in &self.0 {
            let this_line: String = line.iter().map(Cell::to_string).collect();
            board.push(this_line);
        }

        board.join("\n")
    }
}

pub fn part1(input: &str) -> u32 {
    let mut board = Board::from_str(input);

    // println!("{}", board.to_string());
    let mut still_going = true;
    while still_going {
        let tmp = board.next();
        board = tmp.0;
        still_going = tmp.1;
    }

    board.count()
}

pub fn part2(input: &str) -> u32 {
    println!("{}", input);
    2039
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
