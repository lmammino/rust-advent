use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cave<'a> {
    name: &'a str,
    is_small: bool,
    is_end: bool,
    is_start: bool,
}

impl<'a> From<&'a str> for Cave<'a> {
    fn from(name: &'a str) -> Self {
        Self {
            name,
            is_end: name == "end",
            is_start: name == "start",
            is_small: name.chars().all(|c| c.is_lowercase()),
        }
    }
}

#[derive(Debug, Clone)]
struct OpenPath<'a> {
    steps: Vec<Cave<'a>>,
    visited_small_caves: HashSet<Cave<'a>>,
    visited_a_small_cave_twice: bool,
}

impl<'a> OpenPath<'a> {
    fn from(first_cave: &'a str) -> Self {
        let first_cave: Cave<'a> = first_cave.into();

        OpenPath {
            steps: vec![first_cave.clone()],
            visited_small_caves: vec![first_cave].into_iter().collect(),
            visited_a_small_cave_twice: false,
        }
    }

    fn current_cave(&self) -> &Cave<'a> {
        self.steps.last().unwrap()
    }

    fn try_add_cave(
        &self,
        next_cave: &Cave<'a>,
        can_visit_a_small_cave_twice: bool,
    ) -> Result<Self, &'static str> {
        if next_cave.is_start {
            return Err("is start");
        }

        if (self.visited_a_small_cave_twice || !can_visit_a_small_cave_twice)
            && self.visited_small_caves.contains(next_cave)
            && next_cave.is_small
        {
            return Err("already visited");
        }

        let mut extended_open_path = self.clone();
        // if next_cave is small, add it to the list of visited ones
        if next_cave.is_small {
            let is_first_visit = extended_open_path
                .visited_small_caves
                .insert(next_cave.clone());

            if !is_first_visit {
                extended_open_path.visited_a_small_cave_twice = true;
            }
        }
        extended_open_path.steps.push(next_cave.clone());

        Ok(extended_open_path)
    }
}

#[derive(Debug)]
struct CavePaths<'a> {
    adj: HashMap<Cave<'a>, HashSet<Cave<'a>>>,
}

impl<'a> From<&'a str> for CavePaths<'a> {
    fn from(s: &'a str) -> Self {
        let mut adj: HashMap<Cave<'a>, HashSet<Cave<'a>>> = Default::default();

        for line in s.lines() {
            let (source, dest) = line.split_once("-").unwrap();
            let source: Cave<'a> = source.into();
            let dest: Cave<'a> = dest.into();

            let s = adj.entry(source.clone()).or_default();
            s.insert(dest.clone());
            // paths are biderectional so we need to add also the opposite connection
            let s = adj.entry(dest).or_default();
            s.insert(source);
        }

        CavePaths { adj }
    }
}

impl<'a> CavePaths<'a> {
    fn visit_all(&self, can_visit_a_small_cave_twice: bool) -> Vec<Vec<Cave<'a>>> {
        let mut paths: Vec<Vec<Cave<'a>>> = vec![];

        // keeps track of all the open paths and associates a set of the visited paths to them to avoid loops
        let mut open_paths: Vec<OpenPath> = vec![OpenPath::from("start")];

        while let Some(current_path) = open_paths.pop() {
            let current_cave = current_path.current_cave();

            let adj_caves = match self.adj.get(current_cave) {
                None => continue,
                Some(adj_caves) => adj_caves,
            };

            for next_cave in adj_caves {
                let new_path =
                    match current_path.try_add_cave(next_cave, can_visit_a_small_cave_twice) {
                        Err(_) => continue,
                        Ok(new_path) => new_path,
                    };

                if next_cave.is_end {
                    paths.push(new_path.steps);
                } else {
                    open_paths.push(new_path);
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

    impl<'a> FromIterator<&'a str> for OpenPath<'a> {
        fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
            OpenPath {
                steps: iter.into_iter().map(|s| s.into()).collect(),
                visited_a_small_cave_twice: false,
                visited_small_caves: Default::default(),
            }
        }
    }

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

        let expected: Vec<OpenPath<'_>> = expected
            .lines()
            .map(|line| line.split(',').collect())
            .collect();

        for path in &expected {
            assert!(paths.contains(&path.steps));
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
