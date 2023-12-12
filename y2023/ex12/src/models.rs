use nom::{
    branch::alt,
    character::complete::{char, u32},
    combinator::{complete, map},
    multi::{many1, separated_list1},
    IResult,
};
use std::{cmp::min, collections::HashMap, fmt::Display, sync::RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Spring {
    Damaged,
    Ok,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Damaged,
            '.' => Self::Ok,
            '?' => Self::Unknown,
            _ => panic!("Invalid spring char: {}", c),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Damaged => write!(f, "#"),
            Spring::Ok => write!(f, "."),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Springs(Vec<Spring>);

impl Display for Springs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in &self.0 {
            write!(f, "{}", spring)?;
        }
        Ok(())
    }
}

impl Springs {
    #[allow(dead_code)]
    pub fn brute_force_arrangements(&self) -> SpringsArrangements {
        SpringsArrangements::new(self.clone())
    }

    fn replace(&mut self, index: usize, spring: Spring) {
        self.0[index] = spring;
    }

    #[allow(dead_code)]
    fn contiguous_sets(&self) -> Vec<u32> {
        let s = &self.to_string();
        let sets: Vec<u32> = s
            .split('.')
            .filter_map(|s| {
                if !s.is_empty() {
                    Some(s.len() as u32)
                } else {
                    None
                }
            })
            .collect();
        sets
    }
}

pub struct SpringsArrangements {
    springs: Springs,
    unknowns: Vec<usize>,
    index: usize,
}

impl SpringsArrangements {
    fn new(springs: Springs) -> Self {
        let unknowns = springs
            .0
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == Spring::Unknown)
            .map(|(i, _)| i)
            .collect();

        Self {
            springs,
            unknowns,
            index: 0,
        }
    }
}

impl Iterator for SpringsArrangements {
    type Item = Springs;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 2_i32.pow(self.unknowns.len() as u32) as usize {
            return None;
        }

        let mut new_springs = self.springs.clone();
        for nbit in 0..self.unknowns.len() {
            // get the i-th bit of the index
            let bit_is_zero = (1 << nbit & self.index) == 0;
            let spring = if bit_is_zero {
                Spring::Ok
            } else {
                Spring::Damaged
            };
            let index_to_replace = self.unknowns[nbit];
            new_springs.replace(index_to_replace, spring);
        }

        self.index += 1;
        Some(new_springs)
    }
}

type CountCacheInner = RwLock<HashMap<(Vec<Spring>, Vec<u32>), usize>>;

pub struct CountCache {
    cache: CountCacheInner,
}

impl CountCache {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    fn get(&self, springs: &[Spring], sets: &[u32]) -> Option<usize> {
        self.cache
            .read()
            .unwrap()
            .get(&(springs.to_vec(), sets.to_vec()))
            .cloned()
    }

    fn set(&self, springs: &[Spring], sets: &[u32], count: usize) {
        self.cache
            .write()
            .unwrap()
            .insert((springs.to_vec(), sets.to_vec()), count);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub springs: Springs,
    pub contiguous_sets: Vec<u32>,
}

impl Record {
    pub fn unfold(self, times: usize) -> Self {
        let mut new_springs = self.springs.0.clone();
        for _ in 0..(times - 1) {
            new_springs.push(Spring::Unknown);
            new_springs.extend(self.springs.0.iter().cloned());
        }
        let mut new_contiguous_sets = self.contiguous_sets.clone();
        for _ in 0..(times - 1) {
            new_contiguous_sets.extend(self.contiguous_sets.iter().cloned());
        }

        Self {
            springs: Springs(new_springs),
            contiguous_sets: new_contiguous_sets,
        }
    }

    #[allow(dead_code)]
    pub fn count_solutions_brute_force(&self) -> usize {
        self.springs
            .brute_force_arrangements()
            .filter(|s| s.contiguous_sets() == self.contiguous_sets)
            .count()
    }

    pub fn count_solutions(&self, cache: &CountCache) -> usize {
        self.count_solutions_rec(
            self.springs.0.as_slice(),
            self.contiguous_sets.as_slice(),
            cache,
        )
    }

    // Count the number of solutions recursively by figuring out how many possible solutions there
    // are by trying to place the first spring in all possible positions and then recursing on the remaining springs and sets.
    // Solution inspired by: https://www.youtube.com/watch?v=g3Ms5e7Jdqo
    #[allow(clippy::only_used_in_recursion)]
    fn count_solutions_rec(
        &self,
        remaining_springs: &[Spring],
        remaining_sets: &[u32],
        cache: &CountCache,
    ) -> usize {
        // base case, no more springs to place
        if remaining_springs.is_empty() {
            // if also the sets are empty we found a valid config, so we return 1, otherwise 0
            return if remaining_sets.is_empty() { 1 } else { 0 };
        }

        // base case2, no more sets to fill
        if remaining_sets.is_empty() {
            // all the remaining springs must be ok or unknown (if they are damaged we can't fill the remaining sets)
            // if they are unknown we can assume a valid configuration is when all the unknown are turned into ok
            return if remaining_springs.contains(&Spring::Damaged) {
                0
            } else {
                1
            };
        }

        // check if we already computed this configuration
        if let Some(count) = cache.get(remaining_springs, remaining_sets) {
            return count;
        }

        let mut num_solutions = 0;

        // now we have 2 recursive cases:
        // CASE 1: the first spring in the remaining springs is ok or unknown
        // CASE 2: the first spring in the remaining springs is either damaged or unknown
        // note that they are not mutually exclusive (if the first spring is unknown we have to account for both cases)

        // CASE 1
        if remaining_springs[0] == Spring::Ok || remaining_springs[0] == Spring::Unknown {
            // we can treat a possible unknown as an ok, so we recurse on the remaining springs and sets
            // (the possibility that it was a damaged spring is accounted in CASE 2)
            num_solutions +=
                self.count_solutions_rec(&remaining_springs[1..], remaining_sets, cache);
        }

        // CASE 2
        if remaining_springs[0] == Spring::Damaged || remaining_springs[0] == Spring::Unknown {
            // this configuration is valid if:
            // 1. There are enough springs left to fill the remaining sets
            if remaining_springs.len() as u32 >= remaining_sets[0] {
                // 2. We look forward N springs (where N is the first number in the remaining sets)
                //    and we need to make sure that there are no ok springs (we are trying to match a sequence of damaged springs)
                if !&remaining_springs[..remaining_sets[0] as usize].contains(&Spring::Ok) {
                    // 3. the next spring after the N springs analysed before needs to be OK (or unknown), because we need to break the sequence of damaged springs
                    //    to match the next set exactly. This is also valid if there are no springs left after the N springs analysed before.
                    if remaining_sets[0] == remaining_springs.len() as u32 // no springs left
                        || remaining_springs[remaining_sets[0] as usize] != Spring::Damaged
                    {
                        // if all the conditions are met we can recurse on the remaining springs and sets
                        // note that we can decrease the sets and shorten the springs accordingly to the size of the matched set
                        let remaining_springs_start_idx =
                            min(remaining_sets[0] as usize + 1, remaining_springs.len());
                        // let remaining_sets_start_idx =
                        num_solutions += self.count_solutions_rec(
                            &remaining_springs[remaining_springs_start_idx..],
                            &remaining_sets[1..],
                            cache,
                        );
                    }
                }
            }
        }

        // store the counts in the cache
        cache.set(remaining_springs, remaining_sets, num_solutions);

        num_solutions
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.springs)?;
        write!(
            f,
            "{}",
            self.contiguous_sets
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )?;
        Ok(())
    }
}

fn parse_springs(input: &str) -> IResult<&str, Springs> {
    let spring = alt((char('#'), char('.'), char('?')));
    let (input, springs) = many1(map(spring, Spring::from))(input)?;

    Ok((input, Springs(springs)))
}

fn parse_contiguius_sets(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, sets) = separated_list1(char(','), u32)(input)?;

    Ok((input, sets))
}

pub fn parse_entry(input: &str) -> IResult<&str, Record> {
    let (input, springs) = parse_springs(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, contiguous_sets) = complete(parse_contiguius_sets)(input)?;

    Ok((
        input,
        Record {
            springs,
            contiguous_sets,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_arrangements() {
        let (_, springs) = parse_springs("???.###").unwrap();
        let solutions: Vec<_> = springs
            .brute_force_arrangements()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solutions.len(), 8);
        assert_eq!(
            solutions,
            vec![
                "....###", "#...###", ".#..###", "##..###", "..#.###", "#.#.###", ".##.###",
                "###.###"
            ]
        );

        let (_, springs) = parse_springs(".??..??...?##.").unwrap();
        let solutions: Vec<_> = springs
            .brute_force_arrangements()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solutions.len(), 32);
        assert_eq!(
            solutions,
            vec![
                "...........##.",
                ".#.........##.",
                "..#........##.",
                ".##........##.",
                ".....#.....##.",
                ".#...#.....##.",
                "..#..#.....##.",
                ".##..#.....##.",
                "......#....##.",
                ".#....#....##.",
                "..#...#....##.",
                ".##...#....##.",
                ".....##....##.",
                ".#...##....##.",
                "..#..##....##.",
                ".##..##....##.",
                "..........###.",
                ".#........###.",
                "..#.......###.",
                ".##.......###.",
                ".....#....###.",
                ".#...#....###.",
                "..#..#....###.",
                ".##..#....###.",
                "......#...###.",
                ".#....#...###.",
                "..#...#...###.",
                ".##...#...###.",
                ".....##...###.",
                ".#...##...###.",
                "..#..##...###.",
                ".##..##...###."
            ]
        );
    }

    #[test]
    fn test_unfold() {
        let (_, record) = parse_entry("???.### 1,1,3").unwrap();
        let unfolded = record.unfold(5);
        assert_eq!(
            unfolded.to_string(),
            "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3".to_string()
        );
    }
}
