fn game<const N: usize>(input: &str) -> usize {
    let start_vals: Vec<usize> = input
        .split(',')
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let mut occurrence = vec![0_usize; N].into_boxed_slice();

    for (turn, n) in start_vals.iter().take(start_vals.len() - 1).enumerate() {
        occurrence[*n] = turn + 1;
    }

    let mut next = *start_vals.last().unwrap();
    let mut new_next;
    for turn in start_vals.len()..N {
        let last_seen = occurrence.get(next).unwrap();
        if *last_seen != 0 {
            new_next = turn - last_seen;
        } else {
            new_next = 0;
        }
        occurrence[next] = turn;
        next = new_next;
    }

    next
}

pub fn part1(input: &str) -> usize {
    game::<2020>(input)
}

pub fn part2(input: &str) -> usize {
    game::<30_000_000>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let expectations_2020 = [
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];

        let expectations_30_000_000 = [
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];

        for (input, output) in expectations_2020.iter() {
            assert_eq!(game::<2020>(*input), *output);
        }

        for (input, output) in expectations_30_000_000.iter() {
            assert_eq!(game::<30_000_000>(*input), *output);
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
