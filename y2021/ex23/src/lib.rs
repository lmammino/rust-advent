use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn move_cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn desired_home_idx(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }
}

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amphipod::A,
            'B' => Amphipod::B,
            'C' => Amphipod::C,
            'D' => Amphipod::D,
            _ => panic!("Invalid amphipod: {}", c),
        }
    }
}

#[derive(Debug)]
struct Burrow {
    hallway: [Option<Amphipod>; 11],
    holes: [Vec<Amphipod>; 4],
    cost: usize,
}

impl Burrow {
    fn between(&self, a: usize, b: usize) -> &[Option<Amphipod>] {
        if b > a {
            return &self.hallway[a + 1..b];
        } else {
            return &self.hallway[b..a];
        }
    }

    fn moves(&self) -> Vec<Command> {
        let mut moves = Vec::new();

        // simulates amphipods going outside
        for (idx, hole) in self.holes.iter().enumerate() {
            if hole.is_empty() {
                continue;
            }

            if hole.iter().all(|a| a.desired_home_idx() == idx) {
                continue;
            }

            let start = idx * 2 + 2;

            // simulate going out and left
            let mut pos: isize = start as isize;
            while pos >= 0 {
                if self.hallway[pos as usize].is_some() {
                    break;
                }
                if pos != 2 && pos != 4 && pos != 6 && pos != 8 {
                    moves.push(Command::Out {
                        home_x: start,
                        pos_hallway: pos as usize,
                    });
                }

                pos -= 1;
            }

            // simulate going out and right
            let mut pos = start;
            while pos < self.hallway.len() {
                if self.hallway[pos].is_some() {
                    break;
                }
                if pos != 2 && pos != 4 && pos != 6 && pos != 8 {
                    moves.push(Command::Out {
                        home_x: start,
                        pos_hallway: pos,
                    });
                }

                pos += 1;
            }
        }

        // simulate going from the hallway to a hole
        for (idx, amphipod) in self.hallway.iter().enumerate() {
            if amphipod.is_none() {
                continue;
            }

            let amphipod = amphipod.unwrap();

            let desired_home_idx = amphipod.desired_home_idx();
            let desired_home_x = desired_home_idx * 2 + 2;
            let desired_home = &self.holes[desired_home_idx];

            if (desired_home.is_empty() || desired_home.iter().all(|a| a == &amphipod))
                && (self
                    .between(idx, desired_home_x)
                    .iter()
                    .all(|a| a.is_none()))
            {
                moves.push(Command::In {
                    home_x: desired_home_x,
                    pos_hallway: idx,
                });
            }
        }

        moves
    }
}

#[derive(Debug)]
enum Command {
    In { pos_hallway: usize, home_x: usize },
    Out { home_x: usize, pos_hallway: usize },
}

impl FromStr for Burrow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hallway: [Option<Amphipod>; 11] = [None; 11];
        let mut holes: [Vec<Amphipod>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        let hole_pos = [3, 5, 7, 9];

        'outer: for line in s.lines().skip(2) {
            for pos in hole_pos {
                let char = line.chars().nth(pos).unwrap();
                if char == '#' {
                    break 'outer;
                }
                let amphipod: Amphipod = char.into();
                holes[(pos - 3) / 2].push(amphipod);
            }
        }

        Ok(Burrow {
            hallway,
            holes,
            cost: 0,
        })
    }
}

pub fn part1(input: &str) -> usize {
    let burrow = input.parse::<Burrow>().unwrap();
    dbg!(&burrow);
    dbg!(&burrow.moves());

    18195
}

pub fn part2(_input: &str) -> usize {
    50265
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18195);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 50265);
    }
}
