use std::collections::{HashMap, HashSet};

pub fn part1_sort(input: &str) -> u32 {
    let mut numbers = input
        .lines()
        .map(|l| {
            l.parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
        })
        .collect::<Vec<u32>>();

    numbers.sort_unstable();

    let mut ones = 0;
    let mut threes = 1; // initiating this at 1 because there is a jump of 3 at the end according to the exercise
    let mut last = 0;

    for number in numbers {
        let diff = number - last;
        if diff == 1 {
            ones += 1;
        }
        if diff == 3 {
            threes += 1;
        }
        last = number;
    }

    ones * threes
}

pub fn part1(input: &str) -> u32 {
    let numbers = input
        .lines()
        .map(|l| {
            l.parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
        })
        .collect::<HashSet<u32>>();

    let mut ones = 0;
    let mut threes = 1; // initiating this at 1 because there is a jump of 3 at the end according to the exercise
    let mut current = 0;

    loop {
        if numbers.contains(&(current + 1)) {
            current += 1;
            ones += 1;
        } else if numbers.contains(&(current + 3)) {
            current += 3;
            threes += 1;
        } else {
            break;
        }
    }

    ones * threes
}

#[derive(Debug)]
struct Cache {
    hit: usize,
    miss: usize,
    cache: HashMap<u32, u32>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            hit: 0,
            miss: 0,
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, key: u32) -> Option<&u32> {
        let val = self.cache.get(&key);
        match val {
            Some(_) => self.hit += 1,
            None => self.miss += 1,
        }
        val
    }

    fn set(&mut self, key: u32, val: u32) {
        self.cache.insert(key, val);
    }
}

fn tribonacci(n: u32, cache: &mut Cache) -> u32 {
    if let Some(x) = cache.get(n) {
        return *x;
    }

    let res = match n {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => tribonacci(n - 1, cache) + tribonacci(n - 2, cache) + tribonacci(n - 3, cache),
    };

    cache.set(n, res);

    res
}

pub fn part2(input: &str) -> u64 {
    let mut numbers = input
        .lines()
        .map(|l| {
            l.parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
        })
        .collect::<Vec<u32>>();
    numbers.sort_unstable();
    let mut combinations_by_chunk: Vec<u32> = Vec::with_capacity(input.len());

    let mut prev_num = 0; // first num is zero (impl)
    let mut elements_in_chunk: usize = 0;

    let cache: &mut Cache = &mut Cache::new();

    for num in numbers {
        if num - prev_num > 2 {
            combinations_by_chunk.push(tribonacci(elements_in_chunk as u32, cache));
            elements_in_chunk = 0;
        } else {
            elements_in_chunk += 1;
        }
        prev_num = num;
    }
    // deals last implicit value (+3 implied at the end)
    combinations_by_chunk.push(tribonacci(elements_in_chunk as u32, cache));

    let result: u64 = combinations_by_chunk.iter().map(|x| *x as u64).product();

    // println!("{:?}", cache);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");

        assert_eq!(part1(input), 1920);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1511207993344);
    }
}
