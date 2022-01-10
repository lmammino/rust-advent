const MAGIC_NUMBER: usize = 20201227;
const DEFAULT_SUBJECT_NUMBER: usize = 7;

fn calculate_loop_size(n: usize) -> usize {
    let mut loop_size = 0;
    let mut acc = 1;
    loop {
        if acc == n {
            return loop_size;
        }

        acc *= DEFAULT_SUBJECT_NUMBER;
        acc %= MAGIC_NUMBER;

        loop_size += 1;
    }
}

fn transform(subject: usize, loop_size: usize) -> usize {
    let mut acc = 1;

    for _ in 0..loop_size {
        acc *= subject;
        acc %= MAGIC_NUMBER;
    }

    acc
}

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let card_pub_key: usize = lines.next().unwrap().parse().unwrap();
    let door_pub_key: usize = lines.next().unwrap().parse().unwrap();

    let card_loop_size = calculate_loop_size(card_pub_key);
    let door_loop_size = calculate_loop_size(door_pub_key);

    transform(
        transform(DEFAULT_SUBJECT_NUMBER, card_loop_size),
        door_loop_size,
    )
}

#[cfg(test)]
mod ex25_tests {
    use super::*;

    #[test]
    fn test_calculate_loop_size() {
        assert_eq!(calculate_loop_size(5764801), 8);
        assert_eq!(calculate_loop_size(17807724), 11);
    }

    #[test]
    fn test_transform() {
        assert_eq!(transform(17807724, 8), 14897079);
        assert_eq!(transform(5764801, 11), 14897079);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 9177528);
    }
}
