use std::str::FromStr;

#[derive(Debug)]
enum Instr {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr_type, amount) = s
            .split_once(' ')
            .ok_or_else(|| String::from("Cannot parse move"))?;
        let amount: usize = amount
            .parse()
            .map_err(|_| String::from("Cannot parse amount as usize"))?;
        let instr = match instr_type {
            "up" => Instr::Up(amount),
            "down" => Instr::Down(amount),
            "forward" => Instr::Forward(amount),
            _ => return Err(String::from("Invalid move")),
        };

        Ok(instr)
    }
}

#[derive(Debug, Default)]
struct Pos {
    horiz: usize,
    depth: usize,
}

impl Pos {
    fn mul(&self) -> usize {
        self.horiz * self.depth
    }

    fn apply(&mut self, instr: Instr) {
        match instr {
            Instr::Up(v) => self.depth -= v,
            Instr::Down(v) => self.depth += v,
            Instr::Forward(v) => self.horiz += v,
        }
    }
}

impl FromIterator<Instr> for Pos {
    fn from_iter<T: IntoIterator<Item = Instr>>(iter: T) -> Self {
        let mut pos = Pos::default();
        for instr in iter {
            pos.apply(instr);
        }
        pos
    }
}

#[derive(Debug, Default)]
struct PosWithAim {
    aim: usize,
    horiz: usize,
    depth: usize,
}

impl PosWithAim {
    fn mul(&self) -> usize {
        self.horiz * self.depth
    }

    fn apply(&mut self, instr: Instr) {
        match instr {
            Instr::Up(v) => self.aim -= v,
            Instr::Down(v) => self.aim += v,
            Instr::Forward(v) => {
                self.horiz += v;
                self.depth += self.aim * v
            }
        }
    }
}

impl FromIterator<Instr> for PosWithAim {
    fn from_iter<T: IntoIterator<Item = Instr>>(iter: T) -> Self {
        let mut pos = PosWithAim::default();
        for instr in iter {
            pos.apply(instr);
        }
        pos
    }
}

pub fn part1(input: &str) -> usize {
    let pos: Pos = input.lines().map(|l| l.parse::<Instr>().unwrap()).collect();
    pos.mul()
}

pub fn part2(input: &str) -> usize {
    let pos: PosWithAim = input.lines().map(|l| l.parse::<Instr>().unwrap()).collect();
    pos.mul()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1484118);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1463827010);
    }
}
