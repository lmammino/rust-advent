pub fn part1(input: &str) -> usize {
    let mut num_ones_per_digit: [usize; 12] = [0; 12];
    let mut lines_count: usize = 0;
    for s in input.lines() {
        let digits: Vec<usize> = s.chars().map(|c| if c == '1' { 1 } else { 0 }).collect();
        for (i, n) in digits.iter().enumerate() {
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
            // accumulate in gamma
            gamma += 1;
        } else {
            // accumulate in epsilon
            epsilon += 1;
        }
    }

    gamma * epsilon
}

pub fn part2(_input: &str) -> usize {
    0
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
        assert_eq!(part2(input), 0);
    }
}
