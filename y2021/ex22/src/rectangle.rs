#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Rectangle {
    p0: Point,
    p1: Point,
}

impl Rectangle {
    pub fn new(x0: isize, y0: isize, x1: isize, y1: isize) -> Self {
        Rectangle {
            p0: Point { x: x0, y: y0 },
            p1: Point { x: x1, y: y1 },
        }
    }

    pub fn area(&self) -> usize {
        let x = (self.p1.x - self.p0.x).abs() as usize;
        let y = (self.p1.y - self.p0.y).abs() as usize;
        x * y
    }
}

pub(crate) type RectangleList = Vec<Rectangle>;

pub(crate) fn diff(rectangles: &RectangleList, rectangle: &Rectangle) -> RectangleList {
    let r = cut(rectangles, &rectangle.p0);
    let mut r = cut(&r, &rectangle.p1);
    r.retain(|r| !is_inside(r, &rectangle));
    r
}

pub(crate) fn cut(rectangles: &RectangleList, point: &Point) -> RectangleList {
    let mut partial_result = vec![];
    for r in rectangles {
        if r.p0.x < point.x && point.x < r.p1.x {
            partial_result.push(Rectangle {
                p0: Point {
                    x: r.p0.x,
                    y: r.p0.y,
                },
                p1: Point {
                    x: point.x,
                    y: r.p1.y,
                },
            });
            partial_result.push(Rectangle {
                p0: Point {
                    x: point.x,
                    y: r.p0.y,
                },
                p1: Point {
                    x: r.p1.x,
                    y: r.p1.y,
                },
            });
        } else {
            partial_result.push(r.clone());
        }
    }

    let mut result = vec![];

    for r in partial_result {
        if r.p0.y < point.y && point.y < r.p1.y {
            result.push(Rectangle {
                p0: Point {
                    x: r.p0.x,
                    y: r.p0.y,
                },
                p1: Point {
                    x: r.p1.x,
                    y: point.y,
                },
            });
            result.push(Rectangle {
                p0: Point {
                    x: r.p0.x,
                    y: point.y,
                },
                p1: Point {
                    x: r.p1.x,
                    y: r.p1.y,
                },
            });
        } else {
            result.push(r);
        }
    }

    result
}

/// is r0 inside r1?
pub(crate) fn is_inside(r0: &Rectangle, r1: &Rectangle) -> bool {
    r0.p0.x >= r1.p0.x && r0.p0.y >= r1.p0.y && r0.p1.x <= r1.p1.x && r0.p1.y <= r1.p1.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut() {
        /*
            0,0 1,0  2,0
            +----+----+
            |    |    |
            |    |1,1 |
        0,1 +----+----+ 2,1
            |    |    |
            |    |1,2 |
        0,2 +----+----+ 2,2

        */
        let rectangles = vec![
            Rectangle {
                // this one splits in 4 (cutting right in the middle)
                p0: Point { x: 0, y: 0 },
                p1: Point { x: 2, y: 2 },
            },
            Rectangle {
                // this one will not be cut (cut point at the edge)
                p0: Point { x: 1, y: 1 },
                p1: Point { x: 3, y: 3 },
            },
        ];
        let point = Point { x: 1, y: 1 };
        let result = cut(&rectangles, &point);
        assert_eq!(result.len(), 5);
        let expected_rectangles = vec![
            Rectangle::new(0, 0, 1, 1),
            Rectangle::new(1, 0, 2, 1),
            Rectangle::new(0, 1, 1, 2),
            Rectangle::new(1, 1, 2, 2),
            Rectangle::new(1, 1, 3, 3),
        ];

        for expected_rectangle in expected_rectangles {
            assert!(result.contains(&expected_rectangle));
        }
    }
}
