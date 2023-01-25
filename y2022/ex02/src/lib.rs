#[derive(Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn from_left_col(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissor,
            _ => panic!("Invalid char {c}"),
        }
    }

    fn from_right_col(c: char) -> Self {
        match c {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissor,
            _ => panic!("Invalid char {c}"),
        }
    }

    fn score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }

    fn for_outcome(&self, outcome: Outcome) -> Move {
        use Move::*;
        use Outcome::*;

        match (self, outcome) {
            (x, Draw) => *x,
            (Rock, Win) => Paper,
            (Paper, Win) => Scissor,
            (Scissor, Win) => Rock,
            (Rock, Lose) => Scissor,
            (Paper, Lose) => Rock,
            (Scissor, Lose) => Paper,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Lose => 0,
            Outcome::Win => 6,
            Outcome::Draw => 3,
        }
    }

    fn from_right_col(c: char) -> Self {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Invalid char {c}"),
        }
    }
}

struct Match(Move, Move);

impl Match {
    fn outcome(&self) -> Outcome {
        use Move::*;
        use Outcome::*;

        match self {
            Match(Rock, Rock) => Draw,
            Match(Paper, Paper) => Draw,
            Match(Scissor, Scissor) => Draw,
            Match(Rock, Paper) => Win,
            Match(Paper, Rock) => Lose,
            Match(Scissor, Rock) => Win,
            Match(Rock, Scissor) => Lose,
            Match(Paper, Scissor) => Win,
            Match(Scissor, Paper) => Lose,
        }
    }

    fn from_line(line: &str) -> Self {
        if let Some((a, b)) = line.split_once(' ') {
            return Match(
                Move::from_left_col(a.chars().next().unwrap()),
                Move::from_right_col(b.chars().next().unwrap()),
            );
        }

        panic!("Cannot parse line {line}");
    }

    fn from_line_with_outcome(line: &str) -> Self {
        if let Some((a, b)) = line.split_once(' ') {
            let opponent_move = Move::from_left_col(a.chars().next().unwrap());
            return Match(
                opponent_move,
                Move::for_outcome(
                    &opponent_move,
                    Outcome::from_right_col(b.chars().next().unwrap()),
                ),
            );
        }

        panic!("Cannot parse line {line}");
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let m = Match::from_line(line);
            m.outcome().score() + m.1.score()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let m = Match::from_line_with_outcome(line);
            m.outcome().score() + m.1.score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_example() {
        let input = "A Y
B X
C Z";
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 12855);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 13726);
    }
}
