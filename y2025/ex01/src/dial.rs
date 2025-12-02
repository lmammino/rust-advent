#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Dial {
    pub position: u8,
}

impl Dial {
    pub fn new() -> Self {
        Self { position: 50 }
    }

    pub fn turn_left(&mut self, steps: u16) -> u16 {
        let cycle = 100;

        // 1. Count full revolutions (each full spin hits 0 once)
        let full_wraps = steps / cycle;

        // 2. Check the remaining partial turn
        let remainder = steps % cycle;

        // We hit 0 if we step back enough to reach it.
        // If we are at 50, we need 50 steps to hit 0.
        // If we are at 0, stepping left goes to 99 (doesn't count).
        let extra_wrap = if self.position > 0 && remainder >= self.position as u16 {
            1
        } else {
            0
        };

        // Update position using Rem_Euclid for safety with negative numbers
        self.position = ((self.position as i16 - steps as i16).rem_euclid(cycle as i16)) as u8;

        full_wraps + extra_wrap
    }

    // O(1) Time Complexity
    pub fn turn_right(&mut self, steps: u16) -> u16 {
        let cycle = 100;

        // 1. Count full revolutions
        let full_wraps = steps / cycle;

        // 2. Check the remaining partial turn
        let remainder = steps % cycle;

        // We hit 0 if current + remainder spills over 100
        let extra_wrap = if (self.position as u16 + remainder) >= cycle {
            1
        } else {
            0
        };

        // Update position
        self.position = ((self.position as i16 + steps as i16).rem_euclid(cycle as i16)) as u8;

        full_wraps + extra_wrap
    }
}
