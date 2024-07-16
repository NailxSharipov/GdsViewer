use std::ops::{Add, AddAssign, Neg, Sub};
use i_triangle::i_overlay::i_float::point::IntPoint;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub(crate) fn tangent(&self, r: f32) -> Point {
        let l = (self.x * self.x + self.y * self.y).sqrt();
        let i = r / l;
        Point { x: -i * self.y, y: i * self.x }
    }

    pub(crate) fn with_int_point(point: &IntPoint) -> Self {
        Self { x: point.x as f32, y: point.y as f32 }
    }
}


impl Add for Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl AddAssign for Point {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}