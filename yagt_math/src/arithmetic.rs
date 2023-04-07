use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Arithmetic<T>:
    Neg
    + Add<T, Output = T>
    + Sub<T, Output = T>
    + Mul<T, Output = T>
    + Div<T, Output = T>
    + AddAssign<T>
    + SubAssign<T>
    + MulAssign<T>
    + DivAssign<T>
{
}

impl<T> Arithmetic<T> for T where
    T: Neg
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + AddAssign<T>
        + SubAssign<T>
        + MulAssign<T>
        + DivAssign<T>
{
}

pub trait ArithmeticGroup<T>: Arithmetic<T> + Identity<T> + Zero<T> + Copy {}
impl<T> ArithmeticGroup<T> for T where T: Arithmetic<T> + Identity<T> + Zero<T> + Copy {}

pub trait Identity<T> {
    fn identity() -> T;
}

impl Identity<u32> for u32 {
    fn identity() -> u32 {
        1
    }
}

impl Identity<i32> for i32 {
    fn identity() -> i32 {
        1
    }
}

impl Identity<u64> for u64 {
    fn identity() -> u64 {
        1
    }
}

impl Identity<i64> for i64 {
    fn identity() -> i64 {
        1
    }
}

impl Identity<f32> for f32 {
    fn identity() -> f32 {
        1.0
    }
}

impl Identity<f64> for f64 {
    fn identity() -> f64 {
        1.0
    }
}

pub trait Zero<T> {
    fn zero() -> T;
}

impl Zero<u32> for u32 {
    fn zero() -> u32 {
        0
    }
}

impl Zero<i32> for i32 {
    fn zero() -> i32 {
        0
    }
}

impl Zero<u64> for u64 {
    fn zero() -> u64 {
        0
    }
}

impl Zero<i64> for i64 {
    fn zero() -> i64 {
        0
    }
}

impl Zero<f32> for f32 {
    fn zero() -> f32 {
        0.0
    }
}

impl Zero<f64> for f64 {
    fn zero() -> f64 {
        0.0
    }
}
