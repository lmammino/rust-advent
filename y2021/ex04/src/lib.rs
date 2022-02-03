use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Board {
    rows: [HashSet<u64>; 5],
    columns: [HashSet<u64>; 5],
}

impl Board {
    fn mark(&mut self, num: u64) -> bool {
        let mut winner = false;
        for item in self.rows.iter_mut().chain(self.columns.iter_mut()) {
            item.remove(&num);
            if item.is_empty() {
                winner = true;
            }
        }

        winner
    }

    fn score(&self, final_num: u64) -> u64 {
        self.rows
            .iter()
            .map(|set| set.iter().sum::<u64>())
            .sum::<u64>()
            * final_num
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: [HashSet<u64>; 5] = Default::default();
        let mut columns: [HashSet<u64>; 5] = Default::default();

        for (row_id, line) in s.lines().enumerate() {
            for (col_id, cell) in line.split_whitespace().enumerate() {
                let n: u64 = cell.parse().unwrap();
                rows[row_id].insert(n);
                columns[col_id].insert(n);
            }
        }

        Ok(Board { rows, columns })
    }
}

fn parse_input(input: &str) -> (impl Iterator<Item = u64> + '_, Vec<Board>) {
    let (raw_seq, raw_boards) = input.split_once("\n\n").unwrap();
    let seq = raw_seq.split(',').map(|n| n.parse::<u64>().unwrap());

    let boards: Vec<Board> = raw_boards
        .split("\n\n")
        .map(|b| b.parse().unwrap())
        .collect();
    (seq, boards)
}

pub fn part1(input: &str) -> u64 {
    let (seq, mut boards) = parse_input(input);

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
    let (seq, mut boards) = parse_input(input);
    let mut active_boards: Vec<&mut Board> = boards.iter_mut().collect();

    for num in seq {
        if active_boards.len() == 1 {
            let board: &mut Board = active_boards[0];
            if board.mark(num) {
                return board.score(num);
            }
        }

        active_boards = active_boards
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
