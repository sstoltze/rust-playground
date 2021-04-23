use num::traits::{One, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct Fraction<I> {
    pub numerator: I,
    pub denominator: I,
}

fn gcd<I: Rem<Output = I> + Zero + Ord + Clone>(x: I, y: I) -> I {
    let mut x = x;
    let mut y = y;
    while y != I::zero() {
        let t = y.clone();
        y = x % y;
        x = t;
    }
    x
}

impl<I: Rem<Output = I> + Ord + Zero + Sub<Output = I> + Div<Output = I> + Clone> Fraction<I> {
    pub fn new(numerator: I, denominator: I) -> Fraction<I> {
        if denominator == I::zero() {
            panic!("Denominator must not be zero.")
        }
        let mut gcd = gcd(numerator.clone(), denominator.clone());
        if denominator < I::zero() {
            gcd = I::zero() - gcd;
        }
        Fraction {
            numerator: numerator / gcd.clone(),
            denominator: denominator / gcd,
        }
    }

    pub fn inverse_prob(&self) -> Fraction<I> {
        Fraction::new(
            self.denominator.clone() - self.numerator.clone(),
            self.denominator.clone(),
        )
    }
}

impl<
        I: Add<Output = I>
            + Mul<Output = I>
            + Rem<Output = I>
            + Ord
            + Zero
            + Sub<Output = I>
            + Div<Output = I>
            + Clone,
    > Add for Fraction<I>
{
    type Output = Fraction<I>;

    fn add(self, rhs: Self) -> Self {
        let numerator =
            self.numerator * rhs.denominator.clone() + self.denominator.clone() * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl<
        I: Sub<Output = I>
            + Mul<Output = I>
            + Rem<Output = I>
            + Ord
            + Zero
            + Sub<Output = I>
            + Div<Output = I>
            + Clone,
    > Sub for Fraction<I>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let numerator =
            self.numerator * rhs.denominator.clone() - self.denominator.clone() * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl<
        I: Mul<Output = I> + Rem<Output = I> + Ord + Zero + Sub<Output = I> + Div<Output = I> + Clone,
    > Mul for Fraction<I>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl<I: fmt::Display> fmt::Display for Fraction<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[derive(Debug)]
pub struct ParseFractionError;

impl<
        I: FromStr + Rem<Output = I> + Ord + Zero + Sub<Output = I> + Div<Output = I> + One + Clone,
    > FromStr for Fraction<I>
{
    type Err = ParseFractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(s);
        if s == "0" || s == "1" {
            return match s.parse::<I>() {
                Ok(numerator) => Ok(Fraction::new(numerator, I::one())),
                _ => Err(ParseFractionError),
            };
        }
        let numbers: Vec<&str> = s.split('/').collect();
        dbg!(&numbers);
        let numerator = numbers[0].parse::<I>();
        let denominator = numbers[1].parse::<I>();
        match (numerator, denominator) {
            (Ok(n), Ok(d)) => Ok(Fraction::new(n, d)),
            _ => Err(ParseFractionError),
        }
    }
}

impl<I: Ord + Mul<Output = I> + Clone> PartialOrd for Fraction<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: Ord + Mul<Output = I> + Clone> Ord for Fraction<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator.clone() * other.denominator.clone())
            .cmp(&(self.denominator.clone() * other.numerator.clone()))
    }
}

impl<I: PartialEq + Mul<Output = I> + Clone> PartialEq for Fraction<I> {
    fn eq(&self, other: &Self) -> bool {
        (self.numerator.clone() * other.denominator.clone())
            == (self.denominator.clone() * other.numerator.clone())
    }
}

impl<I: Eq + Mul<Output = I> + Clone> Eq for Fraction<I> {}
