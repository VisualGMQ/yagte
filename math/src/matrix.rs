use crate::arithmetic::*;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Matrix<T, const COL: usize, const ROW: usize> {
    data: [[T; ROW]; COL],
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Matrix<T, COL, ROW> {
    pub fn ones() -> Self {
        Self {
            data: [[T::identity(); ROW]; COL],
        }
    }

    pub fn from_row(datas: &[T]) -> Self {
        let mut result = Self::zeros();
        for (i, elem) in datas.iter().enumerate() {
            result.set(i % COL, i / COL, *elem);
        }
        result
    }

    pub fn from_row_vecs(rows: &[RowVector<T, COL>; ROW]) -> Self {
        Matrix::<T, ROW, COL>::from_col_vecs(&rows.map(|row| row.transpose())).transpose()
    }

    pub fn from_col(datas: &[T]) -> Self {
        let mut result = Self::zeros();
        for (i, elem) in datas.iter().enumerate() {
            result.set(i / ROW, i % ROW, *elem);
        }
        result
    }

    pub fn from_col_vecs(rows: &[ColVector<T, ROW>; COL]) -> Self {
        Self {
            data: std::array::from_fn(|i| rows[i].to_array()),
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

    pub fn mul_each(&self, rhs: Matrix<T, COL, ROW>) -> Self {
        let mut result = Self::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(x, y, self.get(x, y) * rhs.get(x, y));
            }
        }
        result
    }
}

impl<T: ArithmeticGroup<T> + PartialEq> Matrix<T, 2, 2> {
    pub fn det(&self) -> T {
        self.get(0, 0) * self.get(1, 1) - self.get(1, 0) * self.get(0, 1)
    }

    pub fn inv(&self) -> Option<Self> {
        let d = self.det();
        if d == T::zero() {
            None
        } else {
            Some(
                Self::from_row(&[
                    self.get(1, 1),
                    -self.get(0, 1),
                    -self.get(1, 0),
                    self.get(0, 0),
                ]) / d,
            )
        }
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

impl<T: ArithmeticGroup<T>, const COMMON: usize, const ROW: usize, const COL: usize>
    Mul<Matrix<T, COL, COMMON>> for Matrix<T, COMMON, ROW>
{
    type Output = Matrix<T, COL, ROW>;

    fn mul(self, rhs: Matrix<T, COL, COMMON>) -> Self::Output {
        let mut result = Self::Output::zeros();
        for j in 0..ROW {
            for k in 0..COL {
                let mut sum = T::zero();
                for i in 0..COMMON {
                    sum += self.get(i, j) * rhs.get(k, i);
                }
                result.set(k, j, sum);
            }
        }
        result
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Mul<T> for Matrix<T, COL, ROW> {
    type Output = Matrix<T, COL, ROW>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Self::Output::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(x, y, self.get(x, y) * rhs);
            }
        }
        result
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

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Div<T> for Matrix<T, COL, ROW> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut result = Self::zeros();
        for x in 0..COL {
            for y in 0..ROW {
                result.set(x, y, self.get(x, y) / rhs);
            }
        }
        result
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> DivAssign for Matrix<T, COL, ROW> {
    fn div_assign(&mut self, rhs: Self) {
        for x in 0..COL {
            for y in 0..ROW {
                self.set(x, y, self.get(x, y) / rhs.get(x, y));
            }
        }
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> DivAssign<T>
    for Matrix<T, COL, ROW>
{
    fn div_assign(&mut self, rhs: T) {
        for x in 0..COL {
            for y in 0..ROW {
                self.set(x, y, self.get(x, y) / rhs);
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

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Mul<Matrix<T, COL, ROW>> for (T,) {
    type Output = Matrix<T, COL, ROW>;

    fn mul(self, rhs: Matrix<T, COL, ROW>) -> Self::Output {
        rhs * self.0
    }
}

impl<T: ArithmeticGroup<T>, const COL: usize, const ROW: usize> Neg for Matrix<T, COL, ROW> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ((-T::identity()),) * self
    }
}

impl<T: ArithmeticGroup<T>, const LEN: usize> MulAssign<Matrix<T, LEN, LEN>>
    for Matrix<T, LEN, LEN>
{
    fn mul_assign(&mut self, rhs: Matrix<T, LEN, LEN>) {
        *self = *self * rhs;
    }
}

pub type Mat22 = Matrix<f32, 2, 2>;
pub type Mat33 = Matrix<f32, 3, 3>;
pub type Mat44 = Matrix<f32, 4, 4>;

impl<const COL: usize, const ROW: usize> Matrix<f32, COL, ROW> {
    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr().cast()
    }
}

#[rustfmt::skip]
impl<T: ArithmeticGroup<T>> Matrix<T, 4, 4> {
    pub fn from_coordination(
        x: Vector3<T>,
        y: Vector3<T>,
        z: Vector3<T>,
        position: Vector3<T>,
    ) -> Self {
        Self::from_row(&[
                x.x(),     y.x(),     z.x(),  position.x(),
                x.y(),     y.y(),     z.y(),  position.y(),
                x.z(),     y.z(),     z.z(),  position.z(),
            T::zero(), T::zero(), T::zero(), T::identity(),
        ])
    }
}

pub type ColVector<T, const LEN: usize> = Matrix<T, 1, LEN>;
pub type RowVector<T, const LEN: usize> = Matrix<T, LEN, 1>;

impl<T: ArithmeticGroup<T>, const LEN: usize> ColVector<T, LEN> {
    pub fn new(datas: [T; LEN]) -> Self {
        Self::from_col(&datas)
    }

    pub fn length_sqrd(&self) -> T {
        let mut sum = T::zero();
        for i in 0..LEN {
            sum += self[i] * self[i];
        }

        sum
    }

    pub fn dot(&self, rhs: &Self) -> T {
        let mut sum = T::zero();
        for i in 0..LEN {
            sum += self[i] * rhs[i];
        }
        sum
    }

    pub fn to_array(&self) -> [T; LEN] {
        self.data[0]
    }
}

impl<const LEN: usize> ColVector<f64, LEN> {
    pub fn length(&self) -> f64 {
        self.length_sqrd().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn normalize_self(&mut self) {
        *self /= self.length();
    }
}

impl<const LEN: usize> ColVector<f32, LEN> {
    pub fn length(&self) -> f32 {
        self.length_sqrd().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn normalize_self(&mut self) {
        *self /= self.length();
    }
}

impl<T: ArithmeticGroup<T>, const LEN: usize> Index<usize> for ColVector<T, LEN> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[0][index]
    }
}

impl<T: ArithmeticGroup<T>, const LEN: usize> IndexMut<usize> for ColVector<T, LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[0][index]
    }
}

pub type Vector<T, const LEN: usize> = ColVector<T, LEN>;

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

impl<T: ArithmeticGroup<T>> Vector2<T> {
    pub fn x_axis() -> Self {
        Self::from_xy(T::identity(), T::zero())
    }
    pub fn y_axis() -> Self {
        Self::from_xy(T::zero(), T::identity())
    }

    pub fn from_xy(x: T, y: T) -> Self {
        Self::new([x, y])
    }

    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }

    pub fn cross(&self, rhs: &Self) -> T {
        self.x() * rhs.y() - self.y() * rhs.x()
    }
}

impl<T: ArithmeticGroup<T>> Vector3<T> {
    pub fn x_axis() -> Self {
        Self::from_xyz(T::identity(), T::zero(), T::zero())
    }
    pub fn y_axis() -> Self {
        Self::from_xyz(T::zero(), T::identity(), T::zero())
    }
    pub fn z_axis() -> Self {
        Self::from_xyz(T::zero(), T::zero(), T::identity())
    }

    pub fn from_xyz(x: T, y: T, z: T) -> Self {
        Self::new([x, y, z])
    }

    pub fn from_vec2(v: Vector2<T>) -> Self {
        Self::from_xyz(v.x(), v.y(), T::identity())
    }

    pub fn from_vec4(v: Vector4<T>) -> Self {
        Self::from_xyz(v.x(), v.y(), v.z())
    }

    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }

    pub fn z(&self) -> T {
        self[2]
    }

    pub fn xy(&self) -> Vector2<T> {
        Vector2::from_xy(self.x(), self.y())
    }

    pub fn xz(&self) -> Vector2<T> {
        Vector2::from_xy(self.x(), self.z())
    }

    pub fn yz(&self) -> Vector2<T> {
        Vector2::from_xy(self.y(), self.z())
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }
}

impl<T: ArithmeticGroup<T>> Vector4<T> {
    pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self::new([x, y, z, w])
    }

    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }

    pub fn z(&self) -> T {
        self[2]
    }

    pub fn w(&self) -> T {
        self[3]
    }

    pub fn xy(&self) -> Vector2<T> {
        Vector2::from_xy(self.x(), self.y())
    }

    pub fn xz(&self) -> Vector2<T> {
        Vector2::from_xy(self.x(), self.z())
    }

    pub fn yz(&self) -> Vector2<T> {
        Vector2::from_xy(self.y(), self.z())
    }

    pub fn xyz(&self) -> Vector3<T> {
        Vector3::from_xyz(self.x(), self.y(), self.z())
    }
}

impl<T: ArithmeticGroup<T>> From<Vector3<T>> for Vector4<T> {
    fn from(value: Vector3<T>) -> Self {
        Self::from_xyzw(value.x(), value.y(), value.z(), T::identity())
    }
}

impl<T: ArithmeticGroup<T>> From<Vector2<T>> for Vector4<T> {
    fn from(value: Vector2<T>) -> Self {
        Self::from_xyzw(value.x(), value.y(), T::zero(), T::identity())
    }
}

impl<T: ArithmeticGroup<T>> From<Vector2<T>> for Vector3<T> {
    fn from(value: Vector2<T>) -> Self {
        Self::from_xyz(value.x(), value.y(), T::zero())
    }
}

impl<T: ArithmeticGroup<T>> From<Vector3<T>> for Vector2<T> {
    fn from(value: Vector3<T>) -> Self {
        Self::from_xy(value.x(), value.y())
    }
}

impl<T: ArithmeticGroup<T>> From<Vector4<T>> for Vector3<T> {
    fn from(value: Vector4<T>) -> Self {
        Self::from_xyz(value.x(), value.y(), value.z())
    }
}

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
