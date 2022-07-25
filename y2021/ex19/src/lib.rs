use std::str::FromStr;

#[derive(Debug)]
struct Scanner {
    position: Option<Point3D>,
    beacons: Vec<Point3D>,
}

#[derive(Debug)]
struct Point3D(i32, i32, i32);

impl FromStr for Point3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        let z: i32 = parts.next().unwrap().parse().unwrap();

        Ok(Point3D(x, y, z))
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .map(|section| {
            let beacons: Vec<Point3D> = section
                .lines()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect();

            Scanner {
                position: None,
                beacons,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let scanners = parse_input(input);

    dbg!(scanners);
    0
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 0);
    }
}
