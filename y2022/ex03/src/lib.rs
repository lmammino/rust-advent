use std::str::FromStr;

mod by3;

fn priority(c: char) -> u64 {
    match c {
        'a'..='z' => ((c as u32) - 96) as u64,
        'A'..='Z' => ((c as u32) - 38) as u64,
        _ => panic!("Invalid char {c}"),
    }
}

fn find_badge(r1: Rucksack, r2: Rucksack, r3: Rucksack) -> char {
    for c in r1.0 {
        if r2.0.contains(&c) && r3.0.contains(&c) {
            return c;
        }
    }
    panic!("Cannot find badge");
}

struct Rucksack(Vec<char>);

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

impl Rucksack {
    fn compartments(&self) -> (&[char], &[char]) {
        let l = self.0.len();
        (&(self.0[0..l / 2]), &(self.0[l / 2..l]))
    }

    fn common_item(&self) -> char {
        let (c_a, c_b) = self.compartments();
        for c in c_a {
            if c_b.contains(c) {
                return *c;
            }
        }
        panic!("Rucksack does not have a common item")
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let r: Rucksack = l.parse().unwrap();
            priority(r.common_item())
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    by3::By3Iter::new(input.lines().map(|l| l.parse::<Rucksack>().unwrap()))
        .map(|(r1, r2, r3)| find_badge(r1, r2, r3))
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7811);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2639);
    }
}
