// use std::collections::{HashMap, HashSet};

// struct Visitor {
//     counter: usize,
//     visited: HashSet<String>,
//     parents_index: HashMap<&str, Vec<&str>>,
// }

// impl Visitor {
// fn new(parents_index: HashMap<&str, Vec<&str>>) -> Self {
//     Visitor {
//         counter: 0,
//         visited: HashSet::new(),
//         parents_index,
//     }
// }

//     fn visit(&mut self, parents_index: HashMap<&str, Vec<&str>>, node: String) -> usize {
//         self.counter += 1;

//         if self.counter < 10 {
//             self.visit(parents_index);
//         }

// TODO: implement. For every parent visit it. If it is not visited, counter + 1 and go on

//         self.counter
//     }
// }

pub fn part1(input: &str) -> usize {
    // let mut parents_index: HashMap<&str, Vec<&str>> = HashMap::new();
    // // ...

    // let mut counter = 0;
    // let mut visited: HashSet<&str> = HashSet::new();

    // IDEA creates an instance of visitor and use the visit method on it

    println!("{}", input);
    289

    // visitor.visit(parents_index, "first node")
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
