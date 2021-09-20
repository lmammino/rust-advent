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

fn inner_eval<I: Iterator<Item = char>>(iter: &mut I) -> u64 {
    let mut acc = 0;
    let mut op = Op::Add;

    while let Some(c) = iter.next() {
        match c {
            ('0'..='9') => {
                let right = (c as u64) - ('0' as u64); // from WHATEWZ encoding to numeric value
                acc = op.apply(acc, right);
            }
            '*' | '+' => {
                op = c.into();
            }
            // This was a sub expression
            ')' => return acc,
            // Start a sub process
            '(' => {
                let right = inner_eval(iter);
                // take the result as value to apply
                acc = op.apply(acc, right);
            }
            _ => {
                // ignore every other character
            }
        }
    }

    acc
}

fn eval(s: &str) -> u64 {
    let mut it = s.chars();
    inner_eval(&mut it)
    // TODO: add a check if "it" is ended
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(eval).sum()
    // 701339185745
}

fn math_with_priority(input: &str) -> u64 {
    match input.find(')') {
        Some(i) => {
            let j = input[..i].rfind('(').unwrap();
            let subresult = math_with_priority(&input[(j + 1)..i]);
            let new_string = format!("{}{}{}", &input[..j], subresult, &input[(i + 1)..]);
            math_with_priority(&new_string)
        }
        _ => input
            .split("*")
            .map(|expr| {
                expr.split("+")
                    .map(|x| x.parse::<u64>().unwrap())
                    .sum::<u64>()
            })
            .product(),
    }
}

pub fn part2(input: &str) -> u64 {
    input.replace(" ","").lines().map(math_with_priority).sum()
    // 4208490449905
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
    fn test_math_with_priority() {
        assert_eq!(math_with_priority("2+3*9"), 45);
        assert_eq!(math_with_priority("9*2+3"), 45);
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
