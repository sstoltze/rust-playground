use num::cast::FromPrimitive;
use num::{One, Zero};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Position<I> {
    x: I,
    y: I,
    z: I,
}

impl<I> Position<I> {
    pub fn new(x: I, y: I, z: I) -> Self {
        Position { x, y, z }
    }
}

impl<I: Add<Output = I> + Sub<Output = I> + Mul<Output = I> + Copy> Position<I> {
    pub fn distance_squared(&self, other: &Self) -> I {
        let x_dist = self.x - other.x;
        let y_dist = self.y - other.y;
        let z_dist = self.z - other.z;
        x_dist * x_dist + y_dist * y_dist + z_dist * z_dist
    }
}

#[derive(Debug, Clone)]
pub struct Vector<I> {
    x: I,
    y: I,
    z: I,
}

impl<I> Vector<I> {
    pub fn new(x: I, y: I, z: I) -> Self {
        Vector { x, y, z }
    }
}

impl<I: Mul<Output = I> + Copy> Vector<I> {
    pub fn scale(self, c: I) -> Self {
        Vector::new(self.x * c, self.y * c, self.z * c)
    }
}

impl<I: Add<Output = I> + Zero + One + Div<Output = I> + Mul + Copy + FromPrimitive> Vector<I> {
    pub fn average(c: Vec<Self>) -> Option<Self> {
        if !c.is_empty() {
            let len = I::from_usize(c.len()).unwrap();
            Some(c.into_iter().sum::<Vector<I>>().scale(I::one() / len))
        } else {
            None
        }
    }
}

impl<I: Add<Output = I>> Add for Vector<I> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<I: Sub<Output = I>> Sub for Vector<I> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<I: Add<Output = I> + Zero> Sum for Vector<I> {
    fn sum<A: Iterator<Item = Self>>(iter: A) -> Self {
        iter.fold(Vector::new(I::zero(), I::zero(), I::zero()), Add::add)
    }
}
