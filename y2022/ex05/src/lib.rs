mod parser;
use parser::*;

#[derive(Eq, PartialEq, Debug)]
pub struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn apply_move(&mut self, mov: Move) {
        for _ in 0..mov.quantity {
            let item = self.0.get_mut(mov.from - 1).unwrap().pop().unwrap();
            self.0.get_mut(mov.to - 1).unwrap().push(item);
        }
    }

    fn apply_move_9001(&mut self, mov: Move) {
        let mut popped: Vec<char> = Vec::new();
        for _ in 0..mov.quantity {
            let item = self.0.get_mut(mov.from - 1).unwrap().pop().unwrap();
            popped.push(item);
        }
        for item in popped.iter().rev() {
            self.0.get_mut(mov.to - 1).unwrap().push(*item);
        }
    }

    fn get_top_text(&self) -> String {
        let top_text: String = self.0.iter().map(|s| s.last().unwrap()).collect();
        top_text
    }
}

impl From<Vec<Vec<Option<char>>>> for Stacks {
    fn from(v: Vec<Vec<Option<char>>>) -> Self {
        let mut stacks = Vec::new();
        for _ in 0..v.len() {
            stacks.push(Vec::new());
        }
        for line in v.iter().rev() {
            for (i, crate_) in line.iter().enumerate() {
                if let Some(c) = crate_ {
                    stacks.get_mut(i).unwrap().push(*c);
                }
            }
        }
        Stacks(stacks)
    }
}

pub fn part1(input: &str) -> String {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();

    for mov in moves {
        stacks.apply_move(mov);
    }

    stacks.get_top_text()
}

pub fn part2(input: &str) -> String {
    let (_, (mut stacks, moves)) = parse_input(input).unwrap();

    for mov in moves {
        stacks.apply_move_9001(mov);
    }

    stacks.get_top_text()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "WHTLRMZRC".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "GMPMLWNMG".to_string());
    }
}
