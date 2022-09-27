use crate::amphipod::Amphipod;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Command {
    In {
        pos_hallway: usize,
        home_x: usize,
        amphipod: Amphipod,
    },
    Out {
        home_x: usize,
        pos_hallway: usize,
        amphipod: Amphipod,
    },
}

impl Command {
    fn cost(&self) -> usize {
        match self {
            Command::In {
                amphipod,
                pos_hallway: _,
                home_x: _,
            } => amphipod.move_cost(),
            Command::Out {
                amphipod,
                pos_hallway: _,
                home_x: _,
            } => amphipod.move_cost(),
        }
    }
}

impl Ord for Command {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost()).reverse()
    }
}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
