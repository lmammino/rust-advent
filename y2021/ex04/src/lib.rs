use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Board {
    rows: [HashSet<u8>; 5],
    columns: [HashSet<u8>; 5],
}

impl Board {
    fn mark(&mut self, num: u8) -> bool {
        let mut winner = false;
        for row in self.rows.iter_mut() {
            row.remove(&num);
            if row.is_empty() {
                winner = true;
            }
        }
        for col in self.columns.iter_mut() {
            col.remove(&num);
            if col.is_empty() {
                winner = true;
            }
        }

        winner
    }

    fn score(&self, final_num: u8) -> u64 {
        let mut score = 0_u64;
        for row in self.rows.iter() {
            for num in row {
                score += *num as u64;
            }
        }
        score * final_num as u64
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: [HashSet<u8>; 5] = [
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
        ];
        let mut columns: [HashSet<u8>; 5] = [
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
            HashSet::default(),
        ];

        for (row_id, line) in s.lines().enumerate() {
            for (col_id, cell) in line.split_whitespace().enumerate() {
                let n: u8 = cell.parse().unwrap();
                rows[row_id].insert(n);
                columns[col_id].insert(n);
            }
        }

        Ok(Board { rows, columns })
    }
}

pub fn part1(input: &str) -> u64 {
    let (raw_seq, raw_boards) = input.split_once("\n\n").unwrap();
    let seq = raw_seq.split(',').map(|n| n.parse::<u8>().unwrap());

    let mut boards: Vec<Board> = raw_boards
        .split("\n\n")
        .map(|b| b.parse().unwrap())
        .collect();

    for num in seq {
        for board in boards.iter_mut() {
            if board.mark(num) {
                return board.score(num);
            }
        }
    }

    unreachable!();
}

pub fn part2(input: &str) -> u64 {
    let (raw_seq, raw_boards) = input.split_once("\n\n").unwrap();
    let seq = raw_seq.split(',').map(|n| n.parse::<u8>().unwrap());

    let mut boards: Vec<Board> = raw_boards
        .split("\n\n")
        .map(|b| b.parse().unwrap())
        .collect();
    let mut bb: Vec<&mut Board> = boards.iter_mut().collect();

    for num in seq {
        if bb.len() == 1 {
            let board: &mut Board = bb[0];
            if board.mark(num) {
                return board.score(num);
            }
        }

        bb = bb
            .into_iter()
            .filter_map(|x| if x.mark(num) { None } else { Some(x) })
            .collect();
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_from_str() {
        let s = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

        let board: Board = s.parse().unwrap();
        assert_eq!(board.score(1), 300); // 300 is the sum of all the numbers in the board
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 5685);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 21070);
    }
}
