pub fn part1(input: &str) -> usize {
    let mut num_ones_per_digit: [usize; 12] = [0; 12];
    let mut lines_count: usize = 0;
    for s in input.lines() {
        let digits = s.chars().map(|c| if c == '1' { 1 } else { 0 });
        for (i, n) in digits.enumerate() {
            num_ones_per_digit[i] += n;
        }
        lines_count += 1;
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;

    for num_ones in num_ones_per_digit.iter() {
        gamma <<= 1;
        epsilon <<= 1;
        if *num_ones > (lines_count / 2) {
            // higher number of 1s: accumulates in gamma
            gamma += 1;
        } else {
            // higher number of 0s: accumulates in epsilon
            epsilon += 1;
        }
    }

    gamma * epsilon
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Digit {
    Zero,
    One,
}

impl Digit {
    fn to_usize(self) -> usize {
        match self {
            Digit::Zero => 0,
            Digit::One => 1,
        }
    }
}

pub fn part2(input: &str) -> usize {
    let mut nums: Vec<[Digit; 12]> = Vec::with_capacity(1000);
    for s in input.lines() {
        let digits = s
            .chars()
            .map(|c| if c == '1' { Digit::One } else { Digit::Zero });
        let mut num: [Digit; 12] = [Digit::Zero; 12];
        for (i, n) in digits.enumerate() {
            num[i] = n;
        }
        nums.push(num);
    }

    let mut oxy_gen_candidates: Vec<&[Digit; 12]> = nums.iter().collect();
    let mut i = 0_usize;
    while oxy_gen_candidates.len() > 1 {
        let num_ones = oxy_gen_candidates
            .iter()
            .filter(|n| n[i] == Digit::One)
            .count();
        let most_common_digit = if num_ones >= (oxy_gen_candidates.len() - num_ones) {
            Digit::One
        } else {
            Digit::Zero
        };
        oxy_gen_candidates = oxy_gen_candidates
            .iter()
            .filter(|digits| digits[i] == most_common_digit)
            .cloned()
            .collect();

        i += 1;
    }

    let mut co2_scrubber_candidates: Vec<&[Digit; 12]> = nums.iter().collect();
    let mut i = 0_usize;
    while co2_scrubber_candidates.len() > 1 {
        let num_ones = co2_scrubber_candidates
            .iter()
            .filter(|n| n[i] == Digit::One)
            .count();
        let least_common_digit = if num_ones < (co2_scrubber_candidates.len() - num_ones) {
            Digit::One
        } else {
            Digit::Zero
        };
        co2_scrubber_candidates = co2_scrubber_candidates
            .iter()
            .filter(|digits| digits[i] == least_common_digit)
            .cloned()
            .collect();

        i += 1;
    }

    let mut oxy_gen: usize = 0;
    for d in oxy_gen_candidates[0] {
        oxy_gen <<= 1;
        oxy_gen += d.to_usize();
    }

    let mut co2_scrubber: usize = 0;
    for d in co2_scrubber_candidates[0] {
        co2_scrubber <<= 1;
        co2_scrubber += d.to_usize();
    }

    oxy_gen * co2_scrubber
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1131506);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 7863147);
    }
}
