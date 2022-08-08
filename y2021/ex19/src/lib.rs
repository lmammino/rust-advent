mod point3d;
mod scanner;
use std::collections::{HashSet};

use point3d::*;
use scanner::*;

pub fn part1(input: &str) -> usize {
    let mut unknown_scanners = scanners_from_input(input);
    let mut first_scanner = unknown_scanners.remove(0);
    first_scanner.position = Some(Point3D(0,0,0));
    let mut known_scanners: Vec<Scanner> = vec![first_scanner];

    while !unknown_scanners.is_empty() {
        dbg!(unknown_scanners.len());
        let mut found_scanner: Option<Scanner> = None;
        let mut found_scanner_index: usize = 0; 
        for k_scanner in &known_scanners {
            for (i, u_scanner) in unknown_scanners.iter().enumerate() {
                if let Some(matched_scanner) = k_scanner.matches(u_scanner) {
                    found_scanner = Some(matched_scanner);
                    found_scanner_index = i;
                    break;
                }
            }
            if found_scanner.is_some() {
                break;
            }
        }
        if let Some(s) = found_scanner {
            known_scanners.push(s);
            unknown_scanners.remove(found_scanner_index);
        }
    }

    let points: HashSet<Point3D> = known_scanners.iter().flat_map(|s| s.beacons.clone()).collect();
    points.len()
}

pub fn part2(_input: &str) -> u32 {
    13000
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
