pub fn part1<const T: usize>(input: &str) -> usize {
    let mut num_ones_per_digit: [usize; T] = [0; T];
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

pub fn part2<const T: usize>(input: &str) -> usize {
    let mut nums: Vec<[usize; T]> = Vec::with_capacity(1000);
    for s in input.lines() {
        let digits = s.chars().map(|c| if c == '1' { 1 } else { 0 });
        let mut num: [usize; T] = [0; T];
        for (i, n) in digits.enumerate() {
            num[i] = n;
        }
        nums.push(num);
    }

    let mut oxy_gen_candidates: Vec<&[usize; T]> = nums.iter().collect();
    let mut i = 0_usize;
    while oxy_gen_candidates.len() > 1 {
        let num_ones = oxy_gen_candidates.iter().filter(|n| n[i] == 1).count();
        let most_common_digit = if num_ones >= (oxy_gen_candidates.len() - num_ones) {
            1
        } else {
            0
        };
        // The following could be eventually done this way:
        // oxy_gen_candidates.drain_filter(|digits| digits[i] == most_common_digit);
        // once `drain_filter` gets stable
        oxy_gen_candidates = oxy_gen_candidates
            .iter()
            .filter(|digits| digits[i] == most_common_digit)
            .cloned()
            .collect();

        i += 1;
    }

    let mut co2_scrubber_candidates: Vec<&[usize; T]> = nums.iter().collect();
    let mut i = 0_usize;
    while co2_scrubber_candidates.len() > 1 {
        let num_ones = co2_scrubber_candidates.iter().filter(|n| n[i] == 1).count();
        let least_common_digit = if num_ones < (co2_scrubber_candidates.len() - num_ones) {
            1
        } else {
            0
        };
        co2_scrubber_candidates = co2_scrubber_candidates
            .iter()
            .filter(|digits| digits[i] == least_common_digit)
            .cloned()
            .collect();

        i += 1;
    }

    let to_usize = |acc: usize, d: &usize| (acc << 1) + d;
    let oxy_gen: usize = oxy_gen_candidates[0].iter().fold(0, to_usize);
    let co2_scrubber: usize = co2_scrubber_candidates[0].iter().fold(0, to_usize);

    oxy_gen * co2_scrubber
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_example_part1() {
        assert_eq!(part1::<5>(EXAMPLE_INPUT), 198);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::<12>(INPUT), 1131506);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2::<5>(EXAMPLE_INPUT), 230);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::<12>(INPUT), 7863147);
    }
}
