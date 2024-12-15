use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Div, Neg, Rem};

use num_traits::{AsPrimitive, Euclid};

pub trait Num:
    num_traits::NumAssign
    + Neg<Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
    + Euclid
    + PartialOrd
    + Sum<Self>
    + AsPrimitive<f64>
    + Copy
    + Display
    + Debug
{
    fn abs(self) -> Self;
}

impl Num for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Num for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Num for i128 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Num for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
