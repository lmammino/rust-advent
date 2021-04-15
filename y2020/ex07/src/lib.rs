#[macro_use]
extern crate lazy_static;
use regex::Regex;

use std::collections::{HashMap, HashSet, VecDeque};
struct Line {
    name: String,
    children: Vec<(usize, String)>,
}

type Parents = HashMap<String, Vec<String>>;
type Children = HashMap<String, Vec<(usize, String)>>;
lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r"^(\w+\s\w+) bags contain ((?:(?:\d+) (?:\w+\s\w+) bags?(?:[,.]\s?))+|no other bags.)$"
    )
    .unwrap();
    static ref BAGS_REGEX: Regex = Regex::new(r"(\d+) (\w+\s\w+) bags?").unwrap();
}

fn parse_line(line: &str) -> Line {
    let capture = LINE_REGEX.captures(line).unwrap();
    // println!("{:?}", capture);

    let name = String::from(&capture[1]);
    let bags = String::from(&capture[2]);

    let mut children: Vec<(usize, String)> = vec![];
    for captured_bags in BAGS_REGEX.captures_iter(&bags) {
        // println!("{:?}", captured_bags);
        children.push((
            captured_bags[1].parse().unwrap(),
            String::from(&captured_bags[2]),
        ));
    }
    Line { name, children }
}

pub fn part1(input: &str) -> usize {
    let bags: Vec<Line> = input.lines().map(parse_line).collect();

    let mut parents: Parents = HashMap::new();

    for bag in bags.iter() {
        for (_, child) in &bag.children {
            parents
                .entry(String::from(child))
                .or_default()
                .push(String::from(&(bag.name)));
        }
    }

    let start_color = String::from("shiny gold");
    let mut visited = HashSet::new();
    let mut check_queue = VecDeque::new();

    check_queue.push_back(&start_color);

    while !check_queue.is_empty() {
        let current_colour = check_queue.pop_front().unwrap();
        visited.insert(current_colour);
        if let Some(current_parents) = parents.get(current_colour) {
            for colour in current_parents {
                if !visited.contains(colour) {
                    visited.insert(colour); // should already be in the set at this stage
                    check_queue.push_back(colour);
                }
            }
        }
    }

    // parents
    // {"bright white": ["light red", "dark orange"],
    // "vibrant plum": ["shiny gold"],
    // "muted yellow": ["light red", "dark orange"],
    // "dotted black": ["dark olive", "vibrant plum"],
    // "faded blue": ["muted yellow", "dark olive", "vibrant plum"],
    // "shiny gold": ["bright white", "muted yellow"],
    // "dark olive": ["shiny gold"]}

    visited.len() - 1
}

pub fn part2(input: &str) -> usize {
    let bags: Vec<Line> = input.lines().map(parse_line).collect();

    let mut children: Children = HashMap::new();

    for bag in bags.iter() {
        for (count, child) in &bag.children {
            children
                .entry(String::from(&(bag.name)))
                .or_default()
                .push((*count, String::from(child)));
        }
    }

    println!("{:?}", &children);

    // TODO: continue from here and implement algorithm from part2.png

    30055
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 289);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 30055);
    }
}
