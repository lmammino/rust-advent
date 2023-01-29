use std::{fmt::Display, ops::Deref, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Snafu(i64);

impl From<i64> for Snafu {
    fn from(input: i64) -> Self {
        Self(input)
    }
}

impl FromStr for Snafu {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut val: i64 = 0;
        for (i, c) in input.chars().rev().enumerate() {
            let n: i64 = match c {
                '0' | '1' | '2' => c.to_string().parse().unwrap(),
                '-' => -1,
                '=' => -2,
                _ => return Err(format!("Invalid character: {c}")),
            };

            val += n * 5_i64.pow(i as u32);
        }
        Ok(Snafu(val))
    }
}

impl Deref for Snafu {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars: Vec<char> = Vec::new();
        let mut curr = **self;
        while curr > 0 {
            let n = (curr + 2).rem_euclid(5) - 2;
            match n {
                -2 => chars.push('='),
                -1 => chars.push('-'),
                0 | 1 | 2 => chars.push(n.to_string().chars().next().unwrap()),
                _ => unreachable!(),
            }
            curr = (curr + 2).div_euclid(5);
        }

        let s = chars.iter().rev().collect::<String>();
        write!(f, "{s}")
    }
}

pub fn part1(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(|line| line.parse::<Snafu>().unwrap().0)
        .sum();

    let snafu_sum: Snafu = sum.into();
    snafu_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_snafu_to_string() {
        let cases: Vec<(i64, &str)> = vec![
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (37, "122"),
            (107, "1-12"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];
        for (num, expected) in cases {
            let snafu: Snafu = num.into();
            let result = snafu.to_string();
            assert_eq!(result, expected, "Failed for {num}");
            // reverse the operation
            let snafu2: Snafu = result.parse().unwrap();
            assert_eq!(snafu, snafu2);
        }
    }

    #[test]
    fn test_part1_example() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        assert_eq!(part1(input), "2=-1=0".to_string());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "2-20=01--0=0=0=2-120".to_string());
    }
}
