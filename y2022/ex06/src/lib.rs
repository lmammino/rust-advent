fn are_unique(chars: &[char]) -> bool {
    for (i, c1) in chars.iter().enumerate() {
        for c2 in &chars[(i + 1)..] {
            if c2 == c1 {
                return false;
            }
        }
    }
    true
}

pub fn part1(input: &str) -> usize {
    let mut chars = input.chars();
    let last4: &mut [char] = &mut [
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
    ];
    for (i, c) in chars.enumerate() {
        last4.rotate_left(1);
        last4[3] = c;

        if are_unique(last4) {
            return i + 5; // 4 already scanned + 1 for 1-based indexing;
        }
    }
    panic!("Did not find start-of-packet marker");
}

pub fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_are_unique() {
        assert!(are_unique(&['a', 'b', 'c', 'd']));
        assert!(!are_unique(&['a', 'b', 'a', 'd']));
        assert!(!are_unique(&['a', 'b', 'c', 'a']));
        assert!(!are_unique(&['a', 'b', 'c', 'b']));
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1109);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
