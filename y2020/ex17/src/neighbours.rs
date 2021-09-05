use crate::Point;

fn neighbour_at<const D: usize>(i: usize) -> [i32; D] {
    let mut el = [0_i32; D];
    let mut n = i;

    for v in el.iter_mut().rev() {
        *v = (n as i32 % 3) - 1; // -1 is needed because we offset the digits so that we are in range -1..1
        n /= 3;
    }

    el
}

#[derive(Debug)]
pub struct NeighboursMaker<const D: usize> {
    relative_points: Vec<[i32; D]>,
}

impl<'a, const D: usize> NeighboursMaker<D> {
    pub fn new() -> Self {
        let relative_points = (0..3_i32.pow(D as u32) as usize)
            .map(neighbour_at::<D>)
            .collect();

        NeighboursMaker { relative_points }
    }

    pub fn for_point(&'a self, point: &'a Point<D>) -> Neighbours<'a, D> {
        Neighbours {
            point,
            next_idx: 0,
            include_self: false,
            maker: self,
        }
    }

    pub fn for_point_with_self(&'a self, point: &'a Point<D>) -> Neighbours<'a, D> {
        Neighbours {
            point,
            next_idx: 0,
            include_self: true,
            maker: self,
        }
    }
}

pub struct Neighbours<'a, const D: usize> {
    point: &'a Point<D>,
    next_idx: usize,
    include_self: bool,
    maker: &'a NeighboursMaker<D>,
}

impl<'a, const D: usize> Iterator for Neighbours<'a, D> {
    type Item = Point<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_idx >= self.maker.relative_points.len() {
            return None;
        }

        let mut next_neighbour = self.maker.relative_points.get(self.next_idx).unwrap();
        if !self.include_self && *next_neighbour == [0; D] {
            self.next_idx += 1;
            next_neighbour = self.maker.relative_points.get(self.next_idx).unwrap();
        }

        let next_point = self.point + *next_neighbour;
        self.next_idx += 1;
        Some(next_point)
    }
}

#[cfg(test)]
mod ex17_tests {
    use super::*;

    #[test]
    fn test_neighbour_at_d2_iter() {
        let reference = Point([0, 0]);
        let neighbours_maker: NeighboursMaker<2> = NeighboursMaker::new();

        // without self
        let neighbours: Vec<_> = neighbours_maker.for_point(&reference).collect();
        let expected = vec![
            Point([-1, -1]),
            Point([-1, 0]),
            Point([-1, 1]),
            Point([0, -1]),
            Point([0, 1]),
            Point([1, -1]),
            Point([1, 0]),
            Point([1, 1]),
        ];
        assert_eq!(neighbours, expected);

        // with self
        let neighbours: Vec<_> = neighbours_maker.for_point_with_self(&reference).collect();

        let expected = vec![
            Point([-1, -1]),
            Point([-1, 0]),
            Point([-1, 1]),
            Point([0, -1]),
            Point([0, 0]),
            Point([0, 1]),
            Point([1, -1]),
            Point([1, 0]),
            Point([1, 1]),
        ];
        assert_eq!(neighbours, expected);
    }

    #[test]
    fn test_neighbour_at_d3_iter() {
        let reference = Point([0, 0, 0]);
        let neighbours_maker: NeighboursMaker<3> = NeighboursMaker::new();

        // without self
        let neighbours: Vec<_> = neighbours_maker.for_point(&reference).collect();

        let expected = vec![
            Point([-1, -1, -1]),
            Point([-1, -1, 0]),
            Point([-1, -1, 1]),
            Point([-1, 0, -1]),
            Point([-1, 0, 0]),
            Point([-1, 0, 1]),
            Point([-1, 1, -1]),
            Point([-1, 1, 0]),
            Point([-1, 1, 1]),
            Point([0, -1, -1]),
            Point([0, -1, 0]),
            Point([0, -1, 1]),
            Point([0, 0, -1]),
            Point([0, 0, 1]),
            Point([0, 1, -1]),
            Point([0, 1, 0]),
            Point([0, 1, 1]),
            Point([1, -1, -1]),
            Point([1, -1, 0]),
            Point([1, -1, 1]),
            Point([1, 0, -1]),
            Point([1, 0, 0]),
            Point([1, 0, 1]),
            Point([1, 1, -1]),
            Point([1, 1, 0]),
            Point([1, 1, 1]),
        ];
        assert_eq!(neighbours, expected);

        // with self
        let neighbours: Vec<_> = neighbours_maker.for_point_with_self(&reference).collect();

        let expected = vec![
            Point([-1, -1, -1]),
            Point([-1, -1, 0]),
            Point([-1, -1, 1]),
            Point([-1, 0, -1]),
            Point([-1, 0, 0]),
            Point([-1, 0, 1]),
            Point([-1, 1, -1]),
            Point([-1, 1, 0]),
            Point([-1, 1, 1]),
            Point([0, -1, -1]),
            Point([0, -1, 0]),
            Point([0, -1, 1]),
            Point([0, 0, -1]),
            Point([0, 0, 0]),
            Point([0, 0, 1]),
            Point([0, 1, -1]),
            Point([0, 1, 0]),
            Point([0, 1, 1]),
            Point([1, -1, -1]),
            Point([1, -1, 0]),
            Point([1, -1, 1]),
            Point([1, 0, -1]),
            Point([1, 0, 0]),
            Point([1, 0, 1]),
            Point([1, 1, -1]),
            Point([1, 1, 0]),
            Point([1, 1, 1]),
        ];
        assert_eq!(neighbours, expected);
    }
}
