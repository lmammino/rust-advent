use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct OpenPath<'a> {
    steps: Vec<&'a str>,
    visited_small_caves: HashSet<&'a str>,
    visited_a_small_cave_twice: bool,
}

impl<'a> OpenPath<'a> {
    fn from(first_cave: &'a str) -> Self {
        OpenPath {
            steps: vec![first_cave],
            visited_small_caves: HashSet::from([first_cave]),
            visited_a_small_cave_twice: false,
        }
    }

    fn current_cave(&self) -> &'a str {
        self.steps.last().unwrap()
    }

    fn can_visit(&self, next_cave: &'a str, can_visit_a_small_cave_twice: bool) -> bool {
        if next_cave == "start" {
            return false;
        }

        let is_next_a_small_cave = next_cave.chars().all(|c| c.is_lowercase());
        if is_next_a_small_cave && self.visited_small_caves.contains(&next_cave) {
            if can_visit_a_small_cave_twice {
                return !self.visited_a_small_cave_twice;
            }

            return false;
        }

        true
    }

    fn extend(&self, next_cave: &'a str) -> Self {
        let mut extended_open_path = self.clone();

        // if next_cave is small, add it to the list of visited ones
        if next_cave.chars().all(|c| c.is_lowercase()) {
            let is_first_visit = extended_open_path.visited_small_caves.insert(&next_cave);

            if !is_first_visit {
                extended_open_path.visited_a_small_cave_twice = true;
            }
        }

        extended_open_path.steps.push(next_cave);

        extended_open_path
    }
}

#[derive(Debug)]
struct CavePaths<'a> {
    adj: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> From<&'a str> for CavePaths<'a> {
    fn from(s: &'a str) -> Self {
        let mut adj: HashMap<&'a str, HashSet<&'a str>> = Default::default();

        for line in s.lines() {
            let (source, dest) = line.split_once("-").unwrap();
            let s = adj.entry(source).or_default();
            s.insert(dest);
            // paths are biderectional so we need to add also the opposite connection
            let s = adj.entry(dest).or_default();
            s.insert(source);
        }

        CavePaths { adj }
    }
}

impl<'a> CavePaths<'a> {
    fn visit_all(&self, can_visit_a_small_cave_twice: bool) -> Vec<Vec<&'a str>> {
        let mut paths: Vec<Vec<&'a str>> = vec![];

        // keeps track of all the open paths and associates a set of the visited paths to them to avoid loops
        let mut open_paths: Vec<OpenPath> = vec![OpenPath::from("start")];

        while let Some(current_path) = open_paths.pop() {
            let current_cave = current_path.current_cave();
            if let Some(adj_caves) = self.adj.get(current_cave) {
                for next_cave in adj_caves {
                    if current_path.can_visit(next_cave, can_visit_a_small_cave_twice) {
                        let new_path = current_path.extend(next_cave);

                        if *next_cave == "end" {
                            paths.push(new_path.steps);
                        } else {
                            open_paths.push(new_path);
                        }
                    }
                }
            }
        }

        paths
    }
}

pub fn part1(input: &str) -> usize {
    let caves: CavePaths<'_> = input.into();
    let paths = caves.visit_all(false);
    paths.len()
}

pub fn part2(input: &str) -> usize {
    let caves: CavePaths<'_> = input.into();
    let paths = caves.visit_all(true);
    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme_part1() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let caves: CavePaths<'_> = input.into();
        let paths = caves.visit_all(false);
        assert_eq!(paths.len(), 10);
    }

    #[test]
    fn test_example_part1() {
        let input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let caves: CavePaths<'_> = input.into();
        let paths = caves.visit_all(false);
        assert_eq!(paths.len(), 19);
    }

    #[test]
    fn test_readme_part2() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let caves: CavePaths<'_> = input.into();
        let paths = caves.visit_all(true);

        let expected = "start,A,b,A,b,A,c,A,end
start,A,b,A,b,A,end
start,A,b,A,b,end
start,A,b,A,c,A,b,A,end
start,A,b,A,c,A,b,end
start,A,b,A,c,A,c,A,end
start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,d,b,A,c,A,end
start,A,b,d,b,A,end
start,A,b,d,b,end
start,A,b,end
start,A,c,A,b,A,b,A,end
start,A,c,A,b,A,b,end
start,A,c,A,b,A,c,A,end
start,A,c,A,b,A,end
start,A,c,A,b,d,b,A,end
start,A,c,A,b,d,b,end
start,A,c,A,b,end
start,A,c,A,c,A,b,A,end
start,A,c,A,c,A,b,end
start,A,c,A,c,A,end
start,A,c,A,end
start,A,end
start,b,A,b,A,c,A,end
start,b,A,b,A,end
start,b,A,b,end
start,b,A,c,A,b,A,end
start,b,A,c,A,b,end
start,b,A,c,A,c,A,end
start,b,A,c,A,end
start,b,A,end
start,b,d,b,A,c,A,end
start,b,d,b,A,end
start,b,d,b,end
start,b,end";

        let expected: Vec<Vec<&str>> = expected
            .lines()
            .map(|line| line.split(',').collect())
            .collect();

        for path in &expected {
            assert!(paths.contains(&path));
        }

        assert_eq!(paths.len(), 36);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 5178);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 130094);
    }
}
