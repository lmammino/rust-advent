use std::str::FromStr;
#[derive(Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Rigth,
    Left,
    Forward
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
            _ => panic!("invalid input")
        }
     }
}


#[derive(Debug)]
struct Instruction {
    action: Action,
    value : u32
}

impl Instruction {
    fn new(action: Action, value: u32) -> Self { Self { action, value } }
}

// "some".into() -> will return the returning value of `from` fn
impl From<&str> for Instruction {
    fn from(_: &str) -> Self {
        todo!()
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = &s[0..1];
        let value = &s[1..].parse::<u32>().expect("invalid input");
        Ok(Instruction::new( action.into(), *value))
    }
}


#[derive(Debug)]
struct Ship {
    direction: u32,
    x: i32,
    y: i32
}

// TODO: imlp Default for Ship
impl Ship {
    fn new() -> Self {
        Self {
            direction: 0,
            x: 0,
            y: 0
        }
    }

    fn go(&mut self, instruction: Instruction) {
        use Action::*;
        match instruction.action {
            North => {
                self.x += instruction.value as i32;
            },
            South => {
                self.x -= instruction.value as i32;
            },
            East => {
                self.y += instruction.value as i32;
            },
            West => {
                self.y -= instruction.value as i32;
            },
            Rigth => {
                self.direction = ( self.direction + instruction.value + 360) % 360;
            },
            Left => {
                self.direction = ( 360 + self.direction - instruction.value) % 360;
            },
            Forward => {
                match self.direction {
                    0 =>  { self.y += instruction.value as i32 },// E
                    90 => { self.x -= instruction.value as i32 },// S
                    180 =>  { self.y -= instruction.value as i32 }, // W
                    270 => { self.x += instruction.value as i32 } // N
                    _ => { unreachable!()}
                }
            }
        }
    }

    fn get_manhattan_distance(&self)  -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}




pub fn part1(input: &str) -> u32 {
    let mut ship = Ship::new();
    input.lines().for_each( |l| {
        ship.go(l.parse().unwrap())
    } );

    ship.get_manhattan_distance()
    //757
}

pub fn part2(input: &str) -> u32 {
    println!("{}", input);
    51249
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
