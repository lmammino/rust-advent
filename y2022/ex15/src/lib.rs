#[macro_use]
extern crate lazy_static;

use readings::Pos;
use std::collections::HashSet;
pub mod readings;

fn solve_part1(
    readings: impl Iterator<Item = (readings::Pos, readings::Pos, i64)>,
    y: i64,
) -> usize {
    /*
     *   Using the formula for Manhattan distance: d = |x1 - x2| + |y1 - y2|
     *   We can determine a range of x values that are within the distance of the sensor:
     *
     *   d = |x - x1| + |y - y1|
     *
     *   for a given reading made of a pair with a Sensor (Sx, Sy) and a Beacon (Bx, By) we can determine
     *   the distance SBd between S and B:
     *
     *   SBd = |Sx - Bx| + |Sy - By|
     *
     *   Now, for the given sensor S, all the points that cannot be beacons are those that are within the
     *   distance of the sensor, so, for a generic point (x, y) the following must be true:
     *
     *   SBd <= | Sx - x | + | Sy - y |
     *
     *   If we want to look at a specific line at y value called Y (e.g. y=2000000), we can rewrite the above as:
     *
     *   SBd <= | Sx - x | + delta_y
     *
     *   Where delta_y = | Sy - Y |
     *
     *   At this point we can solve for x and obtaint the following range:
     *
     *   delta_y - SBd + Sx <= x < Sx + SBd - delta_y
     */
    let mut pos: HashSet<i64> = HashSet::new();
    for (sensor, _, distance) in readings {
        let delta_y = (sensor.y - y).abs();

        let range_start = delta_y - distance + sensor.x;
        let range_end = sensor.x + distance - delta_y;
        let range = range_start..range_end;

        for v in range {
            pos.insert(v);
        }
    }

    pos.len()
}

pub fn part1(input: &str) -> usize {
    let readings = readings::parse(input);
    solve_part1(readings, 2000000)
}

pub fn part2(input: &str) -> i64 {
    let readings: Vec<_> = readings::parse(input).collect();
    let mut points: Vec<Pos> = Vec::new();

    /*
        For every reading we extend the distance between the beacon and the sensor by 1.

        Then we get all the points in the perimiter of the sensor detection area (extended by 1).

        Every point here, if it's within the boundary area (0, 4000000), might be our candidate for the distress signal.

        In order to check if it's the actual distress signal we need to check if it's within the distance of all the sensors.
    */
    for (sensor, _, distance) in readings.iter() {
        let ext_distance = distance + 1;
        for d in 0..=ext_distance {
            //
            let p = Pos {
                x: (sensor.x + d),
                y: (sensor.y + ext_distance - d),
            };
            if p.inside_square(0, 4000000) {
                points.push(p);
            }
            let p = Pos {
                x: (sensor.x - d),
                y: (sensor.y + ext_distance - d),
            };
            if p.inside_square(0, 4000000) {
                points.push(p);
            }
            let p = Pos {
                x: (sensor.x + d),
                y: (sensor.y - ext_distance + d),
            };
            if p.inside_square(0, 4000000) {
                points.push(p);
            }
            let p = Pos {
                x: (sensor.x - d),
                y: (sensor.y - ext_distance + d),
            };
            if p.inside_square(0, 4000000) {
                points.push(p);
            }
        }
    }

    let distress_signal = points
        .iter()
        .find(|point| {
            readings.iter().all(|(sensor, _, orig_dist)| {
                let new_dist = point.dist(sensor);
                new_dist > *orig_dist
            })
        })
        .expect("distress signal not found");

    distress_signal.y + distress_signal.x * 4_000_000_i64
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(solve_part1(readings::parse(input), 10), 26);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4876693);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11645454855041);
    }
}
