use itertools::Itertools;

struct Pairs<T: Iterator<Item = X>, X: Clone> {
    iter: T,
    prev: Option<X>,
}

impl<T: Iterator<Item = X>, X: Clone> Pairs<T, X> {
    pub fn new(iter: T) -> Self {
        let prev = None;
        Pairs { iter, prev }
    }
}

impl<T: Iterator<Item = X>, X: Clone> Iterator for Pairs<T, X> {
    type Item = (X, X);

    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.is_none() {
            self.prev = self.iter.next();
        }

        self.prev.as_ref()?;

        let current = self.iter.next();

        current.as_ref()?;

        let prev = self.prev.clone().unwrap();
        self.prev = current.clone();

        Some((prev, current.unwrap()))
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

pub fn part1_iter(input: &str) -> usize {
    let pairs = Pairs::new(input.lines().map(|x| x.parse::<usize>().unwrap()));
    pairs.filter(|(prev, curr)| curr > prev).count()
}

pub fn part1_zip(input: &str) -> usize {
    let numbers = input.lines().map(|x| x.parse::<usize>().unwrap());
    let numbers2 = numbers.clone().skip(1);

    numbers
        .zip(numbers2)
        .filter(|(prev, curr)| curr > prev)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(v1, v2, v3)| v1 + v2 + v3)
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2_example() {
        let input = "607
618
618
617
647
716
769
792";
        assert_eq!(part2(input), 5);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1292);
    }

    #[test]
    fn test_part1_iter() {
        let input = include_str!("../input.txt");
        assert_eq!(part1_iter(input), 1292);
    }

    #[test]
    fn test_part1_zip() {
        let input = include_str!("../input.txt");
        assert_eq!(part1_zip(input), 1292);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1262);
    }

    #[test]
    fn test_pairs() {
        // empty iter
        let iter = "".split(',');
        let mut pairs = Pairs::new(iter);
        assert_eq!(pairs.next(), None);

        // with data
        let iter = "10,11,12,13,14".split(',');
        let mut pairs = Pairs::new(iter);

        assert_eq!(pairs.next(), Some(("10", "11")));
        assert_eq!(pairs.next(), Some(("11", "12")));
        assert_eq!(pairs.next(), Some(("12", "13")));
        assert_eq!(pairs.next(), Some(("13", "14")));
        assert_eq!(pairs.next(), None);
    }
}
