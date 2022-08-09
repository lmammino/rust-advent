use std::{
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point3D(pub i32, pub i32, pub i32);

impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add for &Point3D {
    type Output = Point3D;

    fn add(self, other: Self) -> Point3D {
        Point3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub for &Point3D {
    type Output = Point3D;

    fn sub(self, other: Self) -> Point3D {
        Point3D(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

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
