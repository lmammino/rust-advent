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
        let x = (self.p1.x - self.p0.x).abs();
        let y = (self.p1.y - self.p0.y).abs();
        (x * y) as usize
    }
}

pub(crate) type RectangleList = Vec<Rectangle>;

pub(crate) fn diff(rectangles: &RectangleList, rectangle: &Rectangle) -> RectangleList {
    let r = cut(rectangles, rectangle, &rectangle.p0);
    let mut r = cut(&r, rectangle, &rectangle.p1);
    r.retain(|r| !is_inside(r, rectangle));
    r.sort_by_key(|r| (r.p0.x, r.p0.y, r.p1.x, r.p1.y));

    let mut i = 0;

    while r.len() > 2 && i < r.len() - 1 {
        let r0 = &r[i];
        let r1 = &r[i + 1];
        if (r0.p0.x == r1.p0.x && r0.p1.y == r1.p0.y && r0.p1.x == r1.p1.x)
            || (r0.p1.x == r1.p0.x && r0.p0.y == r1.p1.y && r0.p0.y == r1.p0.y)
        {
            r[i] = Rectangle::new(r0.p0.x, r0.p0.y, r1.p1.x, r1.p1.y);
            r.remove(i + 1);
        } else {
            i += 1;
        }
    }

    r
}

fn touch(r1: &Rectangle, r2: &Rectangle) -> bool {
    let (x1, y1, x2, y2) = (r1.p0.x, r1.p0.y, r1.p1.x, r1.p1.y);
    let (x3, y3, x4, y4) = (r2.p0.x, r2.p0.y, r2.p1.x, r2.p1.y);

    (x1 <= x3 && x3 <= x2 && y1 <= y3 && y3 <= y2)
        || (x1 <= x4 && x4 <= x2 && y1 <= y3 && y3 <= y2)
        || (x1 <= x3 && x3 <= x2 && y1 <= y4 && y4 <= y2)
        || (x1 <= x4 && x4 <= x2 && y1 <= y4 && y4 <= y2)
        || (x3 <= x1 && x1 <= x4 && y3 <= y1 && y1 <= y4)
        || (x3 <= x2 && x2 <= x4 && y3 <= y1 && y1 <= y4)
        || (x3 <= x1 && x1 <= x4 && y3 <= y2 && y2 <= y4)
        || (x3 <= x2 && x2 <= x4 && y3 <= y2 && y2 <= y4)
        || (x1 <= x4 && x4 <= x2 && y3 <= y1 && y4 >= y2)
        || (y1 <= y4 && y4 <= y2 && x3 <= x1 && x4 >= x2)
}

fn cut(rectangles: &RectangleList, rectangle: &Rectangle, point: &Point) -> RectangleList {
    let mut partial_result = vec![];
    for r in rectangles {
        if touch(r, rectangle) && r.p0.x < point.x && point.x < r.p1.x {
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
        if touch(&r, rectangle) && r.p0.y < point.y && point.y < r.p1.y {
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
        let result = cut(&rectangles, &Rectangle::new(1, 1, 2, 2), &point);
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
