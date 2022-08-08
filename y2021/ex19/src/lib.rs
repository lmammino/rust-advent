mod point3d;
mod scanner;
use std::{collections::{HashSet, HashMap}};
use point3d::*;
use scanner::*;

fn process_scanners (mut unknown_scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut first_scanner = unknown_scanners.remove(0);
    first_scanner.position = Some(Point3D(0,0,0));
    let mut known_scanners: Vec<Scanner> = vec![first_scanner];

    while !unknown_scanners.is_empty() {
        let known_scanner_length = known_scanners.len();
        for i in 0..known_scanner_length {
            for j in (0..unknown_scanners.len()).rev() {
                if let Some(matched_scanner) = known_scanners[i].matches(&unknown_scanners[j]) {
                    known_scanners.push(matched_scanner);
                    unknown_scanners.remove(j);
                }
            }
        }
    }

    known_scanners
}

pub fn part1(input: &str) -> usize {
    let unknown_scanners = scanners_from_input(input);
    let known_scanners = process_scanners(unknown_scanners);

    let points: HashSet<Point3D> = known_scanners.iter().flat_map(|s| s.beacons.clone()).collect();
    points.len()
}

pub fn part2(input: &str) -> i32 {
    let unknown_scanners = scanners_from_input(input);
    let known_scanners = process_scanners(unknown_scanners);

    let mut max_distance = 0;
    for s1 in &known_scanners {
        for s2 in &known_scanners {
            let pos1 = s1.position.clone().unwrap();
            let pos2 = s2.position.clone().unwrap();
            let distance = (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs() + (pos1.2 - pos2.2).abs();
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }


    max_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 414);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 13000);
    }
}
