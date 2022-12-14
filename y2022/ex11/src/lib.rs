mod expr;
mod monkey;
mod parser;
use monkey::*;
use parser::*;
use std::collections::HashMap;

fn solve(input: &str, worry_reduction: u64, turns: usize) -> u64 {
    let mut monkeys: Vec<Monkey> = parse_input(input).collect();
    let mut inspections: HashMap<usize, u64> = HashMap::new();
    let max_multiplier: u64 = monkeys.iter().map(|m| m.test_divisible_by).product();

    for _round in 0..turns {
        for monkey_id in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_id].items.len() {
                let monkey = monkeys.get_mut(monkey_id).unwrap();
                let worry = monkey.items.pop_front().unwrap();

                let new_worry =
                    (monkey.operation.eval(worry, worry) % max_multiplier) / worry_reduction;

                let throw_to_monkey = if new_worry % monkey.test_divisible_by == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                let other_monkey = monkeys.get_mut(throw_to_monkey as usize).unwrap();
                other_monkey.items.push_back(new_worry);

                let entry = inspections.entry(monkey_id).or_default();
                *entry += 1;
            }
        }
    }

    // multiple top 2 inspections
    let mut result: Vec<(usize, u64)> = inspections.into_iter().collect();
    result.sort_by_key(|(_, v)| *v);
    result.iter().rev().take(2).map(|(_, v)| *v).product()
}

pub fn part1(input: &str) -> u64 {
    solve(input, 3, 20)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 1, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = include_str!("../input_example.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 10605);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 50830);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 14399640002);
    }
}
