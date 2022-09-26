use crate::amphipod::Amphipod;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub(crate) struct Hole<const DEPTH: usize> {
    amphipods: [Option<Amphipod>; DEPTH],
    size: usize,
}

impl<const DEPTH: usize> Hole<DEPTH> {
    pub(crate) fn new() -> Self {
        Hole {
            amphipods: [None; DEPTH],
            size: 0,
        }
    }

    pub(crate) fn push(&mut self, amphipod: Amphipod) {
        self.amphipods[self.size] = Some(amphipod);
        self.size += 1;
    }

    pub(crate) fn pop(&mut self) -> Option<Amphipod> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        self.amphipods[self.size].take()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub(crate) fn iter_filled(&self) -> impl Iterator<Item = Amphipod> + '_ {
        self.amphipods.iter().filter_map(|a| *a)
    }

    pub(crate) fn len(&self) -> usize {
        self.size
    }
}
