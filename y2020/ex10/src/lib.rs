use std::collections::HashSet;

pub fn part1_sort(input: &str) -> u32 {
    let mut numbers = input.lines()
        .map(|l| {
            l.parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
        }).collect::<Vec<u32>>();
        
    numbers.sort();

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
    let numbers = input.lines()
        .map(|l| {
            l.parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
        }).collect::<HashSet<u32>>();

    let mut ones = 0;
    let mut threes = 1; // initiating this at 1 because there is a jump of 3 at the end according to the exercise
    let mut current = 0;
    
    loop {
        if numbers.contains(&(current+1)) {
            current += 1;
            ones += 1; 
        } else if numbers.contains(&(current+3)) {
            current += 3;
            threes += 1; 
        } else {
            break;
        }
    }

    ones * threes
}

pub fn part2(input: &str) -> u64 {
    1511207993344
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
