use std::str::FromStr;

#[derive(Debug)]
struct LanterFishSim {
    fish: [usize; 9],
    day: usize,
}

impl LanterFishSim {
    pub fn new(fish: [usize; 9]) -> Self {
        LanterFishSim { fish, day: 0 }
    }

    pub fn count_fishes(&self) -> usize {
        self.fish.iter().sum()
    }
}

impl Iterator for LanterFishSim {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.fish[(self.day + 7) % 9] += self.fish[self.day % 9];
        self.day += 1;
        Some(())
    }
}

impl FromStr for LanterFishSim {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let start_fish = input.split(',').map(|x| x.parse::<u8>().unwrap());
        let mut fishes = [0_usize; 9];
        for fish in start_fish {
            fishes[fish as usize] += 1;
        }

        Ok(LanterFishSim::new(fishes))
    }
}

pub fn part1(input: &str) -> usize {
    let mut sim: LanterFishSim = input.parse().unwrap();
    sim.nth(79);
    sim.count_fishes()
}

pub fn part2(input: &str) -> usize {
    let mut sim: LanterFishSim = input.parse().unwrap();
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
        let sim: LanterFishSim = input.parse().unwrap();
        assert_eq!(sim.count_fishes(), 5);
    }

    #[test]
    fn test_days_passing() {
        let input = "3,4,3,1,2";
        let mut sim: LanterFishSim = input.parse().unwrap();
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
