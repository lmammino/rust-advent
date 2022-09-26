#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    pub(crate) fn move_cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    pub(crate) fn desired_home_idx(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    pub(crate) fn all() -> [Amphipod; 4] {
        [Amphipod::A, Amphipod::B, Amphipod::C, Amphipod::D]
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
