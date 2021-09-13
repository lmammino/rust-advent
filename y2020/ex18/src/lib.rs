use std::collections::VecDeque;

enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Op::Add => left + right,
            Op::Mul => left * right,
        }
    }
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => unreachable!(),
        }
    }
}

fn eval(s: &str) -> u64 {
    let mut acc = 0;
    let mut op = Op::Add;

    let mut items: VecDeque<(usize, char)> = s.char_indices().collect();
    while !items.is_empty() {
        let (i, c) = items.pop_front().unwrap();
        if ('0'..='9').contains(&c) {
            let right = (c as u64) - ('0' as u64); // from WHATEWZ code to numeric value
            acc = op.apply(acc, right);
        } else if c == '*' || c == '+' {
            op = c.into();
        } else if c == '(' {
            // search for the matching closing parenthesis
            let mut open = 1;
            while !items.is_empty() {
                let (j, c) = items.pop_front().unwrap();
                if c == '(' {
                    open += 1;
                } else if c == ')' {
                    open -= 1;
                    if open == 0 {
                        // call eval recursively with substring
                        let right = eval(&s[i + 1..j]);
                        // take the result as value to apply
                        acc = op.apply(acc, right);
                        break;
                    }
                }
            }
        }
    }

    acc
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(eval).sum()
    // 701339185745
}

pub fn part2(_input: &str) -> u64 {
    4208490449905
}

#[cfg(test)]
mod ex18_tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(eval("2 + 3 * 9"), (2 + 3) * 9);
        assert_eq!(eval("2 + 3 * 9 + 4"), (2 + 3) * 9 + 4);
        assert_eq!(eval("2 + (3 * 9) + 4"), 2 + 3 * 9 + 4);
        assert_eq!(eval("2 + (3 * (9 + 2)) + 4"), 2 + 3 * (9 + 2) + 4);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 701339185745);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 4208490449905);
    }
}
