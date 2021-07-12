use std::collections::HashMap;

fn game(input: &str, iterations: usize) -> usize {
    let start_vals: Vec<usize> = input
        .split(',')
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let mut occurence: HashMap<usize, usize> = HashMap::with_capacity(iterations);

    for (turn, n) in start_vals[..start_vals.len() - 1].iter().enumerate() {
        occurence.insert(*n, turn + 1);
    }

    let mut next = *start_vals.last().unwrap();
    let mut new_next;
    for turn in start_vals.len()..iterations {
        if let Some(last_seen) = occurence.get(&next) {
            new_next = turn - last_seen;
        } else {
            new_next = 0;
        }
        occurence.insert(next, turn);
        next = new_next;
    }

    next
}

pub fn part1(input: &str) -> usize {
    game(input, 2020)
}

pub fn part2(input: &str) -> usize {
    game(input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let expectations = [
            ("1,3,2", 1, 2020),
            ("2,1,3", 10, 2020),
            ("1,2,3", 27, 2020),
            ("2,3,1", 78, 2020),
            ("3,2,1", 438, 2020),
            ("3,1,2", 1836, 2020),
            // These ones will make the tests horrendously slow -.-"
            // ("0,3,6", 175594, 30000000),
            // ("1,3,2", 2578, 30000000),
            // ("2,1,3", 3544142, 30000000),
            // ("1,2,3", 261214, 30000000),
            // ("2,3,1", 6895259, 30000000),
            // ("3,2,1", 18, 30000000),
            // ("3,1,2", 362, 30000000),
        ];

        for (input, output, iterations) in expectations.iter() {
            assert_eq!(game(*input, *iterations), *output);
        }
    }

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 232);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 18929178);
    }
}
