use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::arithmetic::*;

pub struct Matrix<T, const COL: usize, const ROW: usize> {
    data: [[T; ROW]; COL],
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Matrix<T, COL, ROW> {
    pub fn ones() -> Self {
        Self {
            data: [[T::identity(); ROW]; COL],
        }
    }

    pub fn zeros() -> Self {
        Self {
            data: [[T::zero(); ROW]; COL],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[x][y] = value;
    }

    pub const fn row(&self) -> usize {
        ROW
    }

    pub const fn col(&self) -> usize {
        COL
    }

    pub const fn width(&self) -> usize {
        self.col()
    }

    pub const fn height(&self) -> usize {
        self.row()
    }

    pub fn transpose(&self) -> Matrix<T, ROW, COL> {
        let mut result = Matrix::<T, ROW, COL>::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(y, x, self.get(x, y));
            }
        }
        result
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let mut result = Self::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(x, y, rhs.get(x, y) * self.get(x, y));
            }
        }
        result
    }
}

impl<T: ArithmeticGroup<T>, const LEN: usize> Matrix<T, LEN, LEN> {
    pub fn identity() -> Self {
        let mut result = Self::zeros();
        for i in 0..LEN {
            result.set(i, i, T::identity());
        }
        result
    }

    pub fn det() -> T {
        todo!("not finish");
    }

    pub fn inv() -> Option<Self> {
        todo!("not finish");
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Add for Matrix<T, COL, ROW> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(x, y, self.get(x, y) + rhs.get(x, y));
            }
        }
        result
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Sub for Matrix<T, COL, ROW> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(x, y, self.get(x, y) - rhs.get(x, y));
            }
        }
        result
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Mul for Matrix<T, COL, ROW> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!("do matrix multiple(not each element multiple)");
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Div for Matrix<T, COL, ROW> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut result = Self::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(x, y, self.get(x, y) / rhs.get(x, y));
            }
        }
        result
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> DivAssign for Matrix<T, COL, ROW> {
    fn div_assign(&mut self, rhs: Self) {
        for x in 0..COL {
            for y in 0..ROW {
                self.set(x, y, self.get(x, y) * rhs.get(x, y));
            }
        }
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> AddAssign for Matrix<T, COL, ROW> {
    fn add_assign(&mut self, rhs: Self) {
        for x in 0..COL {
            for y in 0..ROW {
                self.set(x, y, self.get(x, y) + rhs.get(x, y));
            }
        }
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> SubAssign for Matrix<T, COL, ROW> {
    fn sub_assign(&mut self, rhs: Self) {
        for x in 0..COL {
            for y in 0..ROW {
                self.set(x, y, self.get(x, y) - rhs.get(x, y));
            }
        }
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> MulAssign for Matrix<T, COL, ROW> {
    fn mul_assign(&mut self, rhs: Self) {
        todo!("do matrix multiple(not each element multiple)");
    }
}

pub type Mat22 = Matrix<f64, 2, 2>;
pub type Mat33 = Matrix<f64, 3, 3>;
pub type Mat44 = Matrix<f64, 4, 4>;

mod test {
    use super::*;

    #[test]
    fn test() {}
}
