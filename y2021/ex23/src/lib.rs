pub(crate) mod amphipod;
pub(crate) mod burrow;
pub(crate) mod command;
pub(crate) mod hole;

use std::collections::{BinaryHeap, HashMap, VecDeque};

use burrow::Burrow;

fn solve<const DEPTH: usize>(burrow: Burrow<DEPTH>) -> usize {
    let mut min_cost = usize::MAX;

    let mut states: VecDeque<Burrow<DEPTH>> = VecDeque::from([burrow]);
    let mut seen_states: HashMap<Burrow<DEPTH>, usize> = HashMap::new();

    while let Some(state) = states.pop_back() {
        if state.is_final() {
            if state.cost < min_cost {
                min_cost = state.cost;
            }
            continue;
        }

        for command in state.moves() {
            let new_state = state.apply(&command);

            if new_state.cost >= min_cost {
                continue;
            }

            if let Some(seen_cost) = seen_states.get(&new_state) {
                if new_state.cost >= *seen_cost {
                    continue;
                }
            }

            let seen_cost = new_state.cost;
            states.push_back(new_state.clone());
            seen_states.insert(new_state, seen_cost);
        }
    }

    min_cost
}

fn solve_binary_heap<const DEPTH: usize>(burrow: Burrow<DEPTH>) -> usize {
    let mut min_cost = usize::MAX;

    let mut states: BinaryHeap<Burrow<DEPTH>> = BinaryHeap::from([burrow]);
    let mut seen_states: HashMap<Burrow<DEPTH>, usize> = HashMap::new();

    while let Some(state) = states.pop() {
        if state.is_final() {
            if state.cost < min_cost {
                min_cost = state.cost;
            }
            continue;
        }

        for command in state.moves() {
            let new_state = state.apply(&command);

            if new_state.cost >= min_cost {
                continue;
            }

            if let Some(seen_cost) = seen_states.get(&new_state) {
                if new_state.cost >= *seen_cost {
                    continue;
                }
            }

            let seen_cost = new_state.cost;
            states.push(new_state.clone());
            seen_states.insert(new_state, seen_cost);
        }
    }

    min_cost
}

pub fn part1(input: &str) -> usize {
    let burrow = input.parse::<Burrow<2>>().unwrap();
    solve(burrow)
}

pub fn part1_binary_heap(input: &str) -> usize {
    let burrow = input.parse::<Burrow<2>>().unwrap();
    solve_binary_heap(burrow)
}

pub fn part2(input: &str) -> usize {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.insert(3, "  #D#C#B#A#");
    lines.insert(4, "  #D#B#A#C#");

    let new_input = lines.join("\n");
    let burrow = new_input.parse::<Burrow<4>>().unwrap();

    solve(burrow)
}

pub fn part2_binary_heap(input: &str) -> usize {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.insert(3, "  #D#C#B#A#");
    lines.insert(4, "  #D#B#A#C#");

    let new_input = lines.join("\n");
    let burrow = new_input.parse::<Burrow<4>>().unwrap();

    solve_binary_heap(burrow)
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
    fn test_part1_binary_heap() {
        assert_eq!(part1_binary_heap(INPUT), 18195);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 50265);
    }

    #[test]
    fn test_part2_binary_heap() {
        assert_eq!(part2_binary_heap(INPUT), 50265);
    }
}
