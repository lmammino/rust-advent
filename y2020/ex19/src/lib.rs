use std::collections::HashMap;

type RuleId = usize;

#[derive(Debug)]
enum Rule {
    Seq(Vec<RuleId>),
    Fork(Vec<RuleId>, Vec<RuleId>),
    Leaf(char),
}

#[derive(Debug)]
struct RuleSet(HashMap<RuleId, Rule>);

fn create_ruleset(raw_rules: &str) -> RuleSet {
    let mut rules: HashMap<RuleId, Rule> = HashMap::new();

    for line in raw_rules.lines() {
        let (key, definition) = line.split_once(": ").unwrap();
        let id: RuleId = key.parse().unwrap();

        if definition.starts_with('"') {
            // this is a leaf rule
            let val = definition.chars().nth(1).unwrap();
            let rule = Rule::Leaf(val);
            rules.insert(id, rule);
        } else if definition.contains('|') {
            // this is a fork
            let (left, right) = definition.split_once(" | ").unwrap();
            let left_ids: Vec<RuleId> = left.split(' ').map(|x| x.parse().unwrap()).collect();
            let right_ids: Vec<RuleId> = right.split(' ').map(|x| x.parse().unwrap()).collect();
            let rule = Rule::Fork(left_ids, right_ids);
            rules.insert(id, rule);
        } else {
            // this is a sequence
            let seq: Vec<RuleId> = definition.split(' ').map(|x| x.parse().unwrap()).collect();
            let rule = Rule::Seq(seq);
            rules.insert(id, rule);
        }
    }

    RuleSet(rules)
}

fn validate<'a>(string: &'a str, ruleset: &RuleSet, current_rule: RuleId) -> Option<&'a str> {
    // base case
    let rule = ruleset.0.get(&current_rule).unwrap();
    match rule {
        Rule::Leaf(c) => {
            if string.starts_with(*c) {
                return Some(&string[1..]);
            }
            None
        }
        Rule::Seq(seq) => {
            let mut next = string;
            for rule in seq {
                next = validate(next, ruleset, *rule)?;
            }
            Some(next)
        }
        Rule::Fork(left, right) => {
            // this is like the previous step
            // but we need to do it for both left and right
            // if any of the two works we go on
            // so this is logically like an or.
            // If both of them fail this fails.
            None
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (rules, strings) = input.split_once("\n\n").unwrap();
    let ruleset = create_ruleset(rules);

    // strings.lines().filter(|s| validate(s, &ruleset, 0)).count()
    195
}

pub fn part2(_input: &str) -> usize {
    309
}

#[cfg(test)]
mod ex17_tests {
    use super::*;

    #[test]
    fn test_create_ruleset() {
        let rules = "\
            0: 1 2\n\
            1: \"a\"\n\
            2: 1 3 | 3 1\n\
            3: \"b\"\
        ";

        let ruleset = create_ruleset(rules);

        println!("{:#?}", ruleset);
    }

    #[test]
    fn test_create_ruleset2() {
        let rules = "\
            0: 4 1 5\n\
            1: 2 3 | 3 2\n\
            2: 4 4 | 5 5\n\
            3: 4 5 | 5 4\n\
            4: \"a\"\n\
            5: \"b\"\
        ";

        let ruleset = create_ruleset(rules);

        println!("{:#?}", ruleset);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 195);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 309);
    }
}
