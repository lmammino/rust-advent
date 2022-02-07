#[derive(Debug)]
struct LanterFishSim {
    adults: [usize; 7],
    young: [usize; 9],
    day: usize,
}

impl LanterFishSim {
    pub fn new(adults: [usize; 7]) -> Self {
        LanterFishSim {
            adults,
            young: [0; 9],
            day: 0,
        }
    }

    pub fn count_fishes(&self) -> usize {
        let mut count = 0_usize;
        for fish_count in self.adults {
            count += fish_count;
        }
        for fish_count in self.young {
            count += fish_count;
        }
        count
    }
}

impl Iterator for LanterFishSim {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let adult_idx = self.day % 7;
        // new fishes spawn from adults today
        let mut new_young_fishes = self.adults[adult_idx];

        let young_idx = self.day % 9;
        // new fishes spawn from young today (also number of fishes becoming adults)
        let num_young_breeding = self.young[young_idx];
        new_young_fishes += num_young_breeding;

        // increases the number of adults for the current day (young becoming adults)
        self.adults[adult_idx] += num_young_breeding;
        // replaces today with the number of young fishes
        self.young[young_idx] = new_young_fishes;

        self.day += 1;
        Some(())
    }
}

impl FromIterator<u8> for LanterFishSim {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut adults = [0_usize; 7];
        for fish in iter {
            adults[fish as usize] += 1;
        }

        LanterFishSim::new(adults)
    }
}

pub fn part1(input: &str) -> usize {
    let mut sim: LanterFishSim = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
    sim.nth(79);
    sim.count_fishes()
}

pub fn part2(input: &str) -> usize {
    let mut sim: LanterFishSim = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
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
        let sim: LanterFishSim = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
        assert_eq!(sim.count_fishes(), 5);
    }

    #[test]
    fn test_days_passing() {
        let input = "3,4,3,1,2";
        let mut sim: LanterFishSim = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
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
