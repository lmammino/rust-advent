use crate::{amphipod::Amphipod, command::Command, hole::Hole};
use std::{str::FromStr, collections::BinaryHeap};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub(crate) struct Burrow<const DEPTH:usize> {
    hallway: [Option<Amphipod>; 11],
    holes: [Hole<DEPTH>; 4],
    pub cost: usize,
}

impl <const DEPTH:usize> Burrow<DEPTH> {
    fn between(&self, a: usize, b: usize) -> &[Option<Amphipod>] {
        if b > a {
            &self.hallway[a + 1..b]
        } else {
            &self.hallway[b..a]
        }
    }

    pub(crate) fn moves(&self) -> BinaryHeap<Command> {
        let mut moves = BinaryHeap::new();

        // simulates amphipods going outside
        for (idx, hole) in self.holes.iter().enumerate() {
            if hole.is_empty() {
                continue;
            }

            if hole.iter_filled().all(|a| a.desired_home_idx() == idx) {
                continue;
            }

            let amphipod = hole.iter_filled().next().unwrap();

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
                        amphipod
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
                        amphipod,
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

            if (desired_home.is_empty() || desired_home.iter_filled().all(|a| a == amphipod))
                && (self
                    .between(idx, desired_home_x)
                    .iter()
                    .all(|a| a.is_none()))
            {
                moves.push(Command::In {
                    home_x: desired_home_x,
                    pos_hallway: idx,
                    amphipod,
                });
            }
        }

        moves
    }

    pub(crate) fn apply(&self, cmd: &Command) -> Self {
        let mut new_state = (*self).clone();

        let cost;
        match cmd {
            Command::In {
                home_x,
                pos_hallway,
                amphipod:_,
            } => {
                let amphipod = new_state.hallway[*pos_hallway].unwrap();
                new_state.hallway[*pos_hallway] = None;
                cost = amphipod.move_cost()
                    // vert cost
                    * (DEPTH - new_state.holes[*home_x / 2 - 1].len() 
                    // horiz cost
                        + (*home_x as isize - *pos_hallway as isize).unsigned_abs());
                new_state.holes[*home_x / 2 - 1].push(amphipod);
                
            }
            Command::Out {
                home_x,
                pos_hallway,
                amphipod:_,
            } => {
                let amphipod = new_state.holes[*home_x / 2 - 1].pop().unwrap();
                new_state.hallway[*pos_hallway] = Some(amphipod);
                cost = amphipod.move_cost()
                    // vert cost
                    * (DEPTH - new_state.holes[*home_x / 2 - 1].len() 
                    // horiz cost
                        + (*home_x as isize - *pos_hallway as isize).unsigned_abs());
            }
        }

        new_state.cost += cost;
        new_state
    }

    pub(crate) fn is_final(&self) -> bool {
        for amphipod in Amphipod::all() {
            let hole = &self.holes[amphipod.desired_home_idx()];
            if hole.len() != DEPTH {
                return false;
            }

            if hole.iter_filled().any(|a| a != amphipod) {
                return false;
            }
        }

        true
    }
}

impl <const DEPTH: usize> FromStr for Burrow<DEPTH> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hallway: [Option<Amphipod>; 11] = [None; 11];
        let mut holes = [Hole::<DEPTH>::new(), Hole::<DEPTH>::new(), Hole::<DEPTH>::new(), Hole::<DEPTH>::new()];

        let hole_pos = [3, 5, 7, 9];

        'outer: for line in s.lines().rev().skip(1) {
            for pos in hole_pos {
                let char = line.chars().nth(pos).unwrap();
                if char == '.' {
                    break 'outer;
                }
                let amphipod: Amphipod = char.into();
                holes[(pos - 3) / 2].push(amphipod);
            }
        }

        Ok(Burrow::<DEPTH> {
            hallway,
            holes,
            cost: 0,
        })
    }
}

impl <const DEPTH: usize> Ord for Burrow<DEPTH> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl <const DEPTH: usize> PartialOrd for Burrow<DEPTH> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_apply() {
        let burrow = INPUT.parse::<Burrow::<2>>().unwrap();
        assert_eq!(burrow.cost, 0);

    let new_burrow = burrow.apply(&Command::Out {
        home_x: 2,
        pos_hallway: 1,
        amphipod: Amphipod::A
    });

    // A moved 1 up and 1 left
    assert_eq!(new_burrow.cost, 2);

    let new_burrow = new_burrow.apply(&Command::Out {
        home_x: 2,
        pos_hallway: 3,
        amphipod: Amphipod::D,
    });

    // D moved 2 up and 1 right (+3000)
    assert_eq!(new_burrow.cost, 3002);

    let new_burrow = new_burrow.apply(&Command::In {
        home_x: 2,
        pos_hallway: 1,
        amphipod: Amphipod::A
    });

    // A moved 1 right and 2 down (+3)
    assert_eq!(new_burrow.cost, 3005);
    }
}
