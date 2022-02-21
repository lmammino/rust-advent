#![no_std]

use core::str::FromStr;

#[derive(Debug)]
struct LanternFishSim {
    fish: [u64; 9],
}

impl LanternFishSim {
    pub fn new(fish: [u64; 9]) -> Self {
        LanternFishSim { fish }
    }

    pub fn count_fishes(&self) -> u64 {
        self.fish.iter().sum()
    }
}

impl Iterator for LanternFishSim {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let new_young_fish = self.fish[0];

        for i in 1..9 {
            self.fish[i - 1] = self.fish[i]
        }
        self.fish[8] = new_young_fish;
        self.fish[6] += new_young_fish; // re-add the original breeder fish

        Some(())
    }
}

impl FromStr for LanternFishSim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_fish = s.split(',').map(|x| x.parse::<u8>().unwrap());
        let mut fishes = [0_u64; 9];
        for fish in start_fish {
            fishes[fish as usize] += 1;
        }

        Ok(LanternFishSim::new(fishes))
    }
}

pub fn part1(input: &str) -> u64 {
    let mut sim: LanternFishSim = input.parse().unwrap();
    sim.nth(79);
    sim.count_fishes()
}

pub fn part2(input: &str) -> u64 {
    let mut sim: LanternFishSim = input.parse().unwrap();
    sim.nth(255);
    sim.count_fishes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 350149);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1590327954513);
    }

    #[test]
    fn test_creates_sim() {
        let input = "3,4,3,1,2";
        let sim: LanternFishSim = input.parse().unwrap();
        assert_eq!(sim.count_fishes(), 5);
    }

    #[test]
    fn test_days_passing() {
        let input = "3,4,3,1,2";
        let mut sim: LanternFishSim = input.parse().unwrap();
        assert_eq!(sim.count_fishes(), 5); // Initial state: 3,4,3,1,2
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 5); // After  1 day:  2,3,2,0,1
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 6); // After  2 days: 1,2,1,6,0,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 7); // After  3 days: 0,1,0,5,6,7,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 9); // After  4 days: 6,0,6,4,5,6,7,8,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 10); // After  5 days: 5,6,5,3,4,5,6,7,7,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 10); // After  6 days: 4,5,4,2,3,4,5,6,6,7
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 10); // After  7 days: 3,4,3,1,2,3,4,5,5,6
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 10); // After  8 days: 2,3,2,0,1,2,3,4,4,5
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 11); // After  9 days: 1,2,1,6,0,1,2,3,3,4,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 12); // After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 15); // After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 17); // After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 19); // After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 20); // After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 20); // After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 21); // After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 22); // After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
        sim.next().unwrap();
        assert_eq!(sim.count_fishes(), 26); // After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
    }
}
