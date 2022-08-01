use std::collections::HashMap;

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

    // TODO: optimization use the generator crate to do this lazily
    pub fn rotations(&self) -> Vec<Scanner> {
        let mut scanners = vec![];

        let x: Vec<i32> = self.beacons.iter().map(|p| p.0).collect();
        let inv_x = inverse(&x);
        let y: Vec<i32> = self.beacons.iter().map(|p| p.1).collect();
        let inv_y = inverse(&y);
        let z: Vec<i32> = self.beacons.iter().map(|p| p.2).collect();
        let inv_z = inverse(&z);

        scanners.push(self.clone());
        scanners.push(scanner_from_raw_coordinates(&x, &z, &y));
        scanners.push(scanner_from_raw_coordinates(&y, &x, &z));
        scanners.push(scanner_from_raw_coordinates(&y, &z, &x));
        scanners.push(scanner_from_raw_coordinates(&z, &x, &y));
        scanners.push(scanner_from_raw_coordinates(&z, &y, &x));

        // inverting z only
        scanners.push(scanner_from_raw_coordinates(&x, &y, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&x, &inv_z, &y));
        scanners.push(scanner_from_raw_coordinates(&y, &x, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&y, &inv_z, &x));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &x, &y));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &y, &x));

        // inverting y only
        scanners.push(scanner_from_raw_coordinates(&x, &inv_y, &z));
        scanners.push(scanner_from_raw_coordinates(&x, &z, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &x, &z));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &z, &x));
        scanners.push(scanner_from_raw_coordinates(&z, &x, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&z, &inv_y, &x));

        // inverted z and y
        scanners.push(scanner_from_raw_coordinates(&x, &inv_y, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&x, &inv_z, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &x, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &inv_z, &x));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &x, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &inv_y, &x));

        // inverted x, z & y
        scanners.push(scanner_from_raw_coordinates(&inv_x, &inv_y, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&inv_x, &inv_z, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &inv_x, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &inv_z, &inv_x));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &inv_x, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &inv_y, &inv_x));

        // inverted x & y
        scanners.push(scanner_from_raw_coordinates(&inv_x, &inv_y, &z));
        scanners.push(scanner_from_raw_coordinates(&inv_x, &z, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &inv_x, &z));
        scanners.push(scanner_from_raw_coordinates(&inv_y, &z, &inv_x));
        scanners.push(scanner_from_raw_coordinates(&z, &inv_x, &inv_y));
        scanners.push(scanner_from_raw_coordinates(&z, &inv_y, &inv_x));

        // inverted x & z
        scanners.push(scanner_from_raw_coordinates(&inv_x, &y, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&inv_x, &inv_z, &y));
        scanners.push(scanner_from_raw_coordinates(&y, &inv_x, &inv_z));
        scanners.push(scanner_from_raw_coordinates(&y, &inv_z, &inv_x));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &inv_x, &y));
        scanners.push(scanner_from_raw_coordinates(&inv_z, &y, &inv_x));

        // inverted x
        scanners.push(scanner_from_raw_coordinates(&inv_x, &y, &z));
        scanners.push(scanner_from_raw_coordinates(&inv_x, &z, &y));
        scanners.push(scanner_from_raw_coordinates(&y, &inv_x, &z));
        scanners.push(scanner_from_raw_coordinates(&y, &z, &inv_x));
        scanners.push(scanner_from_raw_coordinates(&z, &inv_x, &y));
        scanners.push(scanner_from_raw_coordinates(&z, &y, &inv_x));

        scanners
    }

    /*
    def matches(scanner0, scanner1):
        for i1, s1 in enumerate(rotations(scanner1)):
            cnt = {}
            for p0 in scanner0:
                for p1 in s1:
                    c = diff3(p0,p1)
                    cnt[c] = cnt.get(c,0) + 1
            m = [k for k,v in cnt.items() if v>=12]
            if m:
                return tuple(sum3(x, m[0]) for x in s1), m[0]
        return None, None
         */

    pub fn matches(&self, scanner: &Scanner) -> Option<Scanner> {
        for rotated_scanner in scanner.rotations() {
            let mut matching: HashMap<Point3D, usize> = Default::default();
            for point0 in &self.beacons {
                for point1 in &rotated_scanner.beacons {
                    let key = point0 - point1;
                    let x = matching.entry(key).or_default();
                    *x += 1;

                    // TODO: check if we can do the >= 12 check here
                }
            }

            if let Some((key, _)) = matching.iter().find(|(_, v)| **v >= 12) {
                let normalised_scanner = rotated_scanner.moved(key);
                return Some(normalised_scanner);
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
