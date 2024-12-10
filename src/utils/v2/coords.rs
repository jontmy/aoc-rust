use std::ops::{Add, Mul, Neg, Sub};

use num::PrimInt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinates<I = i32>(I, I)
where
    I: PrimInt;

impl<I> Coordinates<I>
where
    I: PrimInt,
{
    pub fn new(x: I, y: I) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> I {
        self.0
    }

    pub fn y(&self) -> I {
        self.1
    }

    pub fn as_tuple(&self) -> (I, I) {
        (self.0, self.1)
    }
}

impl<I> From<(I, I)> for Coordinates<I>
where
    I: PrimInt,
{
    fn from((x, y): (I, I)) -> Self {
        Self(x, y)
    }
}

impl<I> Into<(I, I)> for Coordinates<I>
where
    I: PrimInt,
{
    fn into(self) -> (I, I) {
        (self.0, self.1)
    }
}

impl<I> Add for Coordinates<I>
where
    I: PrimInt,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<I> Sub for Coordinates<I>
where
    I: PrimInt,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<I> Neg for Coordinates<I>
where
    I: PrimInt + Neg<Output = I>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl<I> Mul<I> for Coordinates<I>
where
    I: PrimInt,
{
    type Output = Self;

    fn mul(self, rhs: I) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
