use std::collections::HashMap;
use generator::*;
use crate::Point3D;

#[derive(Debug, Clone)]
pub struct Scanner {
    pub position: Option<Point3D>,
    pub beacons: Vec<Point3D>,
}

pub fn scanners_from_input(input: &str) -> Vec<Scanner> {
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

fn scanner_from_raw_coordinates(x: &Vec<i32>, y: &Vec<i32>, z: &Vec<i32>) -> Scanner {
    let beacons: Vec<Point3D> = x
        .iter()
        .zip(y.iter())
        .zip(z.iter())
        .map(|((x, y), z)| Point3D(*x, *y, *z))
        .collect();

    Scanner {
        position: None,
        beacons,
    }
}

fn inverse(list: &Vec<i32>) -> Vec<i32> {
    list.iter().map(|x| -x).collect()
}

impl Scanner {
    pub fn moved(&self, point: &Point3D) -> Scanner {
        Scanner {
            position: Some(point.clone()),
            beacons: self.beacons.iter().map(|p| p + point).collect(),
        }
    }

    pub fn rotations(&self) -> Generator<'_, (), Scanner> {
        Gn::new_scoped(move |mut s| {

        let x: Vec<i32> = self.beacons.iter().map(|p| p.0).collect();
        let inv_x = inverse(&x);
        let y: Vec<i32> = self.beacons.iter().map(|p| p.1).collect();
        let inv_y = inverse(&y);
        let z: Vec<i32> = self.beacons.iter().map(|p| p.2).collect();
        let inv_z = inverse(&z);

        s.yield_with(self.clone());
        s.yield_with(scanner_from_raw_coordinates(&x, &z, &y));
        s.yield_with(scanner_from_raw_coordinates(&y, &x, &z));
        s.yield_with(scanner_from_raw_coordinates(&y, &z, &x));
        s.yield_with(scanner_from_raw_coordinates(&z, &x, &y));
        s.yield_with(scanner_from_raw_coordinates(&z, &y, &x));

        // inverting z only
        s.yield_with(scanner_from_raw_coordinates(&x, &y, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&x, &inv_z, &y));
        s.yield_with(scanner_from_raw_coordinates(&y, &x, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&y, &inv_z, &x));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &x, &y));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &y, &x));

        // inverting y only
        s.yield_with(scanner_from_raw_coordinates(&x, &inv_y, &z));
        s.yield_with(scanner_from_raw_coordinates(&x, &z, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &x, &z));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &z, &x));
        s.yield_with(scanner_from_raw_coordinates(&z, &x, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&z, &inv_y, &x));

        // inverted z and y
        s.yield_with(scanner_from_raw_coordinates(&x, &inv_y, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&x, &inv_z, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &x, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &inv_z, &x));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &x, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &inv_y, &x));

        // inverted x, z & y
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &inv_y, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &inv_z, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &inv_x, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &inv_z, &inv_x));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &inv_x, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &inv_y, &inv_x));

        // inverted x & y
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &inv_y, &z));
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &z, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &inv_x, &z));
        s.yield_with(scanner_from_raw_coordinates(&inv_y, &z, &inv_x));
        s.yield_with(scanner_from_raw_coordinates(&z, &inv_x, &inv_y));
        s.yield_with(scanner_from_raw_coordinates(&z, &inv_y, &inv_x));

        // inverted x & z
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &y, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &inv_z, &y));
        s.yield_with(scanner_from_raw_coordinates(&y, &inv_x, &inv_z));
        s.yield_with(scanner_from_raw_coordinates(&y, &inv_z, &inv_x));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &inv_x, &y));
        s.yield_with(scanner_from_raw_coordinates(&inv_z, &y, &inv_x));

        // inverted x
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &y, &z));
        s.yield_with(scanner_from_raw_coordinates(&inv_x, &z, &y));
        s.yield_with(scanner_from_raw_coordinates(&y, &inv_x, &z));
        s.yield_with(scanner_from_raw_coordinates(&y, &z, &inv_x));
        s.yield_with(scanner_from_raw_coordinates(&z, &inv_x, &y));
        s.yield_with(scanner_from_raw_coordinates(&z, &y, &inv_x));

        done!();
        })
    }

    pub fn matches(&self, scanner: &Scanner) -> Option<Scanner> {
        for rotated_scanner in scanner.rotations() {
            let mut matching: HashMap<Point3D, usize> = Default::default();
            for point0 in &self.beacons {
                for point1 in &rotated_scanner.beacons {
                    let key = point0 - point1;
                    // TODO: can we remove this clone? -.-
                    let x = matching.entry(key.clone()).or_default();
                    *x += 1;

                    if *x >= 12 {
                        let normalised_scanner = rotated_scanner.moved(&key);
                        return Some(normalised_scanner);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_matching() {
        let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390";

        let scanners = scanners_from_input(input);

        let first = &scanners[0];
        let second = &scanners[1];

        let matching = first.matches(second).expect("It should match");
        assert_eq!(matching.position.unwrap(), Point3D(68, -1246, -43));
    }
}
