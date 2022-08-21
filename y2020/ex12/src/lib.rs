use std::str::FromStr;
#[derive(Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Rigth,
    Left,
    Forward,
}

impl From<&str> for Action {
    fn from(c: &str) -> Self {
        match c {
            "N" => Action::North,
            "S" => Action::South,
            "E" => Action::East,
            "W" => Action::West,
            "R" => Action::Rigth,
            "L" => Action::Left,
            "F" => Action::Forward,
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: u32,
}

impl Instruction {
    fn new(action: Action, value: u32) -> Self {
        Self { action, value }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = &s[0..1];
        let value = &s[1..].parse::<u32>().expect("invalid input");
        Ok(Instruction::new(action.into(), *value))
    }
}

#[derive(Debug, Default)]
struct Ship {
    direction: u32,
    x: i32,
    y: i32,
}

impl Ship {
    fn go(&mut self, instruction: Instruction) {
        use Action::*;
        match instruction.action {
            North => {
                self.y += instruction.value as i32;
            }
            South => {
                self.y -= instruction.value as i32;
            }
            East => {
                self.x += instruction.value as i32;
            }
            West => {
                self.x -= instruction.value as i32;
            }
            Rigth => {
                self.direction = (self.direction + instruction.value + 360) % 360;
            }
            Left => {
                self.direction = (360 + self.direction - instruction.value) % 360;
            }
            Forward => {
                match self.direction {
                    0 => self.y += instruction.value as i32,   // E
                    90 => self.x -= instruction.value as i32,  // S
                    180 => self.y -= instruction.value as i32, // W
                    270 => self.x += instruction.value as i32, // N
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
    }

    // for part 2
    fn apply_waypoint(&mut self, multiplier: u32, waypoint: &Waypoint) {
        self.x += waypoint.x * multiplier as i32;
        self.y += waypoint.y * multiplier as i32;
    }

    fn get_manhattan_distance(&self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new() -> Self {
        Waypoint { x: 10, y: 1 }
    }

    fn rotate(&mut self, degree: i32) {
        match degree {
            90 => {
                let tmp = self.x;
                self.x = self.y;
                self.y = -(tmp);
            }
            180 => {
                self.x = -(self.x);
                self.y = -(self.y);
            }
            270 => {
                let tmp = self.y;
                self.y = self.x;
                self.x = -(tmp);
            }
            _ => unreachable!(),
        }
    }

    fn transform(&mut self, instruction: Instruction) {
        use Action::*;
        match instruction.action {
            North => {
                self.y += instruction.value as i32;
            }
            South => {
                self.y -= instruction.value as i32;
            }
            East => {
                self.x += instruction.value as i32;
            }
            West => {
                self.x -= instruction.value as i32;
            }
            Rigth => {
                self.rotate(instruction.value as i32);
            }
            Left => {
                self.rotate(((360 - instruction.value) % 360) as i32);
            }
            _ => {
                unreachable!()
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut ship: Ship = Default::default();
    input.lines().for_each(|l| ship.go(l.parse().unwrap()));

    ship.get_manhattan_distance()
}

pub fn part2(input: &str) -> u32 {
    let mut ship: Ship = Default::default();
    let mut waypoint = Waypoint::new();
    input.lines().for_each(|l| {
        let instruction: Instruction = l.parse().unwrap();
        if let Action::Forward = instruction.action {
            ship.apply_waypoint(instruction.value, &waypoint);
        } else {
            waypoint.transform(instruction);
        }
    });

    ship.get_manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 757);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 51249);
    }
}
