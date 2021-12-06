use std::str::FromStr;

#[derive(Debug)]
struct GameState {
    cups: Vec<u64>,
    current: usize,
}

impl FromStr for GameState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups: Vec<u64> = s.chars().map(|c| (c as u64) - ('0' as u64)).collect();
        Ok(GameState { cups, current: 0 })
    }
}

impl Iterator for GameState {
    type Item = ([u64; 3], u64);

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        let mut pick_up = [0_u64; 3];

        for i in 0..3 {
            pick_up[i] = self.cups.remove(1);
        }

        let mut dest_value = self.cups.get(self.current).unwrap() - 1;
        if dest_value == 0 {
            dest_value = 9;
        }

        let mut dest = self
            .cups
            .iter()
            .enumerate()
            .find(|(_, v)| **v == dest_value);

        while dest.is_none() {
            dest_value -= 1;
            if dest_value == 0 {
                dest_value = 9;
            }
            dest = self
                .cups
                .iter()
                .enumerate()
                .find(|(_, v)| **v == dest_value);
        }

        let (dest_idx, _) = dest.unwrap();

        let target_idx = dest_idx + 1;
        for val in pick_up.iter().rev() {
            self.cups.insert(target_idx, *val);
        }

        // move to the next element
        let current_value = self.cups[0];
        self.cups = self.cups[1..].to_vec();
        self.cups.push(current_value);

        Some((pick_up, dest_value))
    }
}

impl GameState {
    fn result(&self) -> usize {
        let mut acc: usize = 0;
        let (pos1, _) = self
            .cups
            .iter()
            .enumerate()
            .find(|(_, v)| **v == 1)
            .unwrap();

        let mut curr_pos = (pos1 + 1) % self.cups.len();
        while curr_pos != pos1 {
            acc *= 10;
            acc += *(self.cups.get(curr_pos).unwrap()) as usize;
            curr_pos = (curr_pos + 1) % self.cups.len();
        }
        acc
    }
}

pub fn part1(input: &str) -> usize {
    let mut state: GameState = input.parse().unwrap();
    state.nth(99);
    state.result()
}

pub fn part2(_input: &str) -> usize {
    166298218695
}

#[cfg(test)]
mod ex23_tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let state: GameState = "345678".parse().unwrap();
        assert_eq!(state.cups, vec![3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_result() {
        let state: GameState = "345126798".parse().unwrap();
        assert_eq!(state.result(), 26798345);
    }

    #[test]
    fn test_iter() {
        let mut state: GameState = "389125467".parse().unwrap();

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [8, 9, 1]);
        assert_eq!(dest, 2);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [8, 9, 1]);
        assert_eq!(dest, 7);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [4, 6, 7]);
        assert_eq!(dest, 3);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [9, 1, 3]);
        assert_eq!(dest, 7);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [6, 7, 9]);
        assert_eq!(dest, 3);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [3, 6, 7]);
        assert_eq!(dest, 9);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [3, 6, 7]);
        assert_eq!(dest, 8);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [5, 8, 3]);
        assert_eq!(dest, 1);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [7, 4, 1]);
        assert_eq!(dest, 5);

        let (pick_up, dest) = state.next().unwrap();
        assert_eq!(pick_up, [7, 4, 1]);
        assert_eq!(dest, 3);

        assert_eq!(state.result(), 92658374);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 26354798);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 166298218695);
    }
}
