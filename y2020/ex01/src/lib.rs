use std::collections::HashSet;

pub fn part1(values: Vec<u32>) -> u32 {
    let idx: HashSet<u32> = values.iter().cloned().collect();
    for v in values {
        let complement = 2020 - v;
        if idx.contains(&complement) {
            return v * complement;
        }
    }
    panic!("Solution not found");
}

pub fn part2(values: Vec<u32>) -> u32 {
    let idx: HashSet<u32> = values.iter().cloned().collect();
    for (i, v) in values.iter().enumerate() {
        for k in values.iter().skip(i) {
            if v + k > 2020 {
                continue;
            }
            let complement = 2020 - (v + k);
            if idx.contains(&complement) {
                return complement * v * k;
            }
        }
    }
    panic!("Solution not found");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        let values: Vec<u32> = input
            .lines()
            .map(|l| {
                l.parse::<u32>()
                    .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
            })
            .collect();
        assert_eq!(part1(values), 866436);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        let values: Vec<u32> = input
            .lines()
            .map(|l| {
                l.parse::<u32>()
                    .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
            })
            .collect();
        assert_eq!(part2(values), 276650720);
    }
}
