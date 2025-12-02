use crate::dial::Dial;

mod dial;
mod parser;

pub fn part1(input: &str) -> usize {
    let mut dial = Dial::new();
    let mut times_dial_at_0 = 0;
    let operations = input.lines().enumerate().map(|(i, line)| {
        parser::parse_dial_op(&mut &*line)
            .expect(format!("Failed to parse line {}", i + 1).as_ref())
    });
    for op in operations {
        match op {
            parser::DialOp::TurnLeft(steps) => dial.turn_left(steps),
            parser::DialOp::TurnRight(steps) => dial.turn_right(steps),
        };
        if dial.position == 0 {
            times_dial_at_0 += 1;
        }
    }
    times_dial_at_0
}

pub fn part2(input: &str) -> u32 {
    let mut dial = Dial::new();
    let mut times_dial_at_0 = 0;
    let operations = input.lines().enumerate().map(|(i, line)| {
        parser::parse_dial_op(&mut &*line)
            .expect(format!("Failed to parse line {}", i + 1).as_ref())
    });
    for op in operations {
        let wraps = match op {
            parser::DialOp::TurnLeft(steps) => dial.turn_left(steps),
            parser::DialOp::TurnRight(steps) => dial.turn_right(steps),
        };
        times_dial_at_0 += wraps as u32;
    }
    times_dial_at_0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1123);
    }

    #[test]
    fn test_part2_example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part2(input), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6695);
    }
}
