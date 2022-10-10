use std::collections::VecDeque;

fn parse_input(input: &str) -> [(i8, i8, i8); 14] {
    let mut res: [(i8, i8, i8); 14] = Default::default();
    let mut lines = input.lines();
    for i in 0..14 {
        for _ in 0..4 {
            lines.next(); // skip 4 lines without moving the iterator
        }

        let div_z = lines.next().unwrap();
        let a: i8 = div_z.split(' ').last().unwrap().parse().unwrap();
        let add_x = lines.next().unwrap();
        let b: i8 = add_x.split(' ').last().unwrap().parse().unwrap();

        for _ in 0..9 {
            lines.next(); // skip 9 lines without moving the iterator
        }

        let add_y = lines.next().unwrap();
        let c: i8 = add_y.split(' ').last().unwrap().parse().unwrap();

        for _ in 0..2 {
            lines.next(); // skip 2 lines without moving the iterator
        }

        res[i] = (a, b, c);
    }

    res
}

pub fn solve<S>(input: &str, selector: S) -> u64
where
    S: FnMut(&(i8, i8)) -> i8,
{
    let params = parse_input(input);
    let mut stack: VecDeque<(i8, usize)> = VecDeque::new();
    let mut ranges: [(i8, i8); 14] = Default::default();

    for (i, (a, b, c)) in params.iter().enumerate() {
        if *a == 1 {
            stack.push_back((*c, i));
        } else {
            let (n, j) = stack.pop_back().unwrap();
            let delta = b + n;
            if delta > 0 {
                ranges[j] = (1, 9 - delta);
            } else {
                ranges[j] = (1 - delta, 9);
            }

            ranges[i] = (ranges[j].0 + delta, ranges[j].1 + delta);
        }
    }

    let res = ranges
        .iter()
        .map(selector)
        .fold(0, |acc, curr| acc * 10 + curr as u64);

    res
}

pub fn part1(input: &str) -> u64 {
    solve(input, |(_min, max)| *max)
}

pub fn part2(input: &str) -> u64 {
    solve(input, |(min, _max)| *min)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_parse_input() {
        let params = parse_input(INPUT);
        assert_eq!(params[0], (1, 14, 12));
        assert_eq!(params[1], (1, 15, 7));
        assert_eq!(params[2], (1, 12, 1));
        assert_eq!(params[3], (1, 11, 2));
        assert_eq!(params[4], (26, -5, 4));
        assert_eq!(params[5], (1, 14, 15));
        assert_eq!(params[6], (1, 15, 11));
        assert_eq!(params[7], (26, -13, 5));
        assert_eq!(params[8], (26, -16, 3));
        assert_eq!(params[9], (26, -8, 9));
        assert_eq!(params[10], (1, 15, 2));
        assert_eq!(params[11], (26, -8, 3));
        assert_eq!(params[12], (26, 0, 3));
        assert_eq!(params[13], (26, -4, 11));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 12996997829399);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11841231117189);
    }
}
