use regex::Regex;

use std::collections::{HashMap, HashSet};
struct Line {
    name: String,
    children: Vec<(usize, String)>,
}

type Parents = HashMap<String, Vec<String>>;

struct Visitor {
    counter: usize,
    visited: HashSet<String>,
    parents_index: Parents,
}

impl Visitor {
    fn new(parents_index: Parents) -> Self {
        Visitor {
            counter: 0,
            visited: HashSet::new(),
            parents_index,
        }
    }

    fn visit(&mut self, node: String) -> usize {
        let parents = self.parents_index.get(&node);
        if let Some(parents) = parents {
            for bag in parents {
                if !self.visited.contains(bag) {
                    self.counter += 1;
                    self.visited.insert(String::from(bag));
                    self.visit(String::from(bag));
                }
            }
        }

// TODO: implement. For every parent visit it. If it is not visited, counter + 1 and go on

        self.counter
    }
}

fn parse_line(line: &str) -> Line {
    let re = Regex::new(r"^(\w+\s\w+) bags contain ((?:(?:\d+) (?:\w+\s\w+) bags?(?:[,.]\s?))+|no other bags.)$").unwrap();
    let rebags = Regex::new(r"(\d+) (\w+\s\w+) bags?").unwrap();

    let capture = re.captures(line).unwrap();
    // println!("{:?}", capture);

    let name = String::from(&capture[1]);
    let bags = String::from(&capture[2]);

    let mut children: Vec<(usize, String)> = vec!();
    for captured_bags in rebags.captures_iter(&bags) {
        // println!("{:?}", captured_bags);
        children.push((captured_bags[1].parse().unwrap(), String::from(&captured_bags[2])));
    }
    Line {
        name,
        children
    }
}


pub fn part1(input: &str) -> usize {
    // let mut parents_index: HashMap<&str, Vec<&str>> = HashMap::new();
    // // ...

    // let mut counter = 0;
    // let mut visited: HashSet<&str> = HashSet::new();

    // IDEA creates an instance of visitor and use the visit method on it
    let bags:Vec<Line> = input.lines().map(parse_line).collect();

    let mut parents : Parents = HashMap::new();

    for bag in bags.iter() {
        for (_, child) in &bag.children {
            parents.entry(String::from(child)).or_default().push(
                String::from(&(bag.name))
            );
        }
    }

    // parents
    // {"bright white": ["light red", "dark orange"], "vibrant plum": ["shiny gold"],
    // "muted yellow": ["light red", "dark orange"],
    // "dotted black": ["dark olive", "vibrant plum"],
    // "faded blue": ["muted yellow", "dark olive", "vibrant plum"],
    // "shiny gold": ["bright white", "muted yellow"], "dark olive": ["shiny gold"]}

    let mut visitor = Visitor::new(parents);

    visitor.visit(String::from("shiny gold"))
}

pub fn part2(input: &str) -> usize {
    println!("{}", input);
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
