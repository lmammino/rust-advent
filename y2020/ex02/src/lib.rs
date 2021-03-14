use regex::Regex;

struct Line {
    min: u32,
    max: u32,
    char: char,
    password: String,
}

fn parse_line(line: &str) -> Line {
    let re = Regex::new(r"^(\d+)-(\d+)\s([a-zA-Z]):\s([a-zA-Z]+)$").unwrap();
    let capture = re.captures(line).unwrap();

    Line {
        min: capture[1].parse().unwrap(),
        max: capture[2].parse().unwrap(),
        char: capture[3].parse().unwrap(),
        password: capture[4].parse().unwrap(),
    }
}

fn validate_line(line: &Line) -> bool {
    let occurences = line.password.matches(line.char).count() as u32;

    occurences >= line.min && occurences <= line.max
}

fn validate_line2(line: &Line) -> bool {
    let start_index = (line.min - 1) as usize;
    let end_index = (line.max - 1) as usize;
    (line.password.chars().nth(start_index).unwrap() == line.char)
        ^ (line.password.chars().nth(end_index).unwrap() == line.char)
}

pub fn part1(input: &str) -> u32 {
    input.lines().map(parse_line).filter(validate_line).count() as u32

    // let mut valid_count = 0;
    // for line in &lines {
    //     let valid = validate_line(parse_line(line));
    //     if valid {
    //         valid_count += 1;
    //     }
    // }
    // valid_count

    // let mut valid_count = 0;
    // for line in &lines {
    //     match validate_line(&parse_line(line)) {
    //         true => valid_count += 1,
    //         false => (),
    //     }
    // }
    // valid_count
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(parse_line).filter(validate_line2).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 454);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 649);
    }
}
