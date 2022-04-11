use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_x, raw_y) = s.split_once(",").unwrap();
        Ok(Point {
            x: raw_x.parse().unwrap(),
            y: raw_y.parse().unwrap(),
        })
    }
}

impl Point {
    //  ___________|__#
    // x = 11
    // px = 14
    // new_px = px - ((x - px) * 2) = 8
    //  ________#__|__*
    fn fold_x(&mut self, x: usize) -> bool {
        if self.x <= x {
            return false;
        }

        self.x = self.x - ((self.x - x) * 2);
        true
    }

    fn fold_y(&mut self, y: usize) -> bool {
        if self.y <= y {
            return false;
        }

        self.y = self.y - ((self.y - y) * 2);
        true
    }

    fn fold(&mut self, fold: &Fold) -> bool {
        match fold {
            Fold::X(x) => self.fold_x(*x),
            Fold::Y(y) => self.fold_y(*y),
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, def) = s.split_once("fold along ").unwrap();
        let (axis, raw_amount) = def.split_once("=").unwrap();
        let amount: usize = raw_amount.parse().unwrap();
        match axis {
            "x" => Ok(Fold::X(amount)),
            "y" => Ok(Fold::Y(amount)),
            _ => Err(()),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (raw_points, raw_folds) = input.split_once("\n\n").unwrap();
    let mut points: Vec<Point> = raw_points
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let first_fold: Fold = raw_folds.lines().next().unwrap().parse().unwrap();

    // apply the first fold to all the points
    points.iter_mut().for_each(|p| {
        p.fold(&first_fold);
    });

    let unique_points: HashSet<Point> = points.into_iter().collect();

    unique_points.len()
}

pub fn part2(input: &str) -> usize {
    let (raw_points, raw_folds) = input.split_once("\n\n").unwrap();
    let mut points: Vec<Point> = raw_points
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let folds = raw_folds.lines().map(|line| line.parse::<Fold>().unwrap());

    for fold in folds {
        points.iter_mut().for_each(|p| {
            p.fold(&fold);
        });
    }

    let unique_points: HashSet<Point> = points.into_iter().collect();

    // Uncomment the following code to prints all the points to read the code:
    //
    // ###  ###   ##  #  # #### ###  #    ###
    // #  # #  # #  # # #  #    #  # #    #  #
    // #  # #  # #    ##   ###  ###  #    #  #
    // ###  ###  #    # #  #    #  # #    ###
    // # #  #    #  # # #  #    #  # #    # #
    // #  # #     ##  #  # #    ###  #### #  #
    //
    // let max_x = unique_points.iter().max_by_key(|p| p.x).unwrap().x;
    // let max_y = unique_points.iter().max_by_key(|p| p.y).unwrap().y;
    // let mut grid: Vec<Vec<char>> = (0..=max_y).map(|_| vec![' '; max_x as usize + 1]).collect();

    // for point in unique_points.iter() {
    //     grid[point.y][point.x] = '#';
    // }

    // for line in grid {
    //     println!("{}", line.as_slice().iter().collect::<String>());
    // }

    unique_points.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_points() {
        let mut p = Point { x: 1, y: 10 };
        let fold = Fold::Y(7);

        p.fold(&fold);

        assert_eq!(p.x, 1); // unchanged
        assert_eq!(p.y, 4);
    }

    #[test]
    fn test_example() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(part1(input), 17);
        assert_eq!(part2(input), 16);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 716);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 97);
    }
}
