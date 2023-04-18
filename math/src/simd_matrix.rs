use std::{
    arch::x86_64::*,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone, Copy)]
union F32x4 {
    simd: __m128,
    data: (f32, f32, f32, f32),
}

#[derive(Clone, Copy)]
pub struct BasicVector<const NUM: usize> {
    data: F32x4,
}

impl<const NUM: usize> BasicVector<NUM> {
    pub fn ones() -> Self {
        Self {
            data: unsafe {
                F32x4 {
                    simd: _mm_set1_ps(1.0),
                }
            },
        }
    }

    pub fn zeros() -> Self {
        Self {
            data: unsafe {
                F32x4 {
                    simd: _mm_setzero_ps(),
                }
            },
        }
    }
}

impl<const NUM: usize> Add for BasicVector<NUM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        BasicVector {
            data: unsafe {
                F32x4 {
                    simd: _mm_add_ps(self.data.simd, rhs.data.simd),
                }
            },
        }
    }
}

impl<const NUM: usize> Sub for BasicVector<NUM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        BasicVector {
            data: unsafe {
                F32x4 {
                    simd: _mm_sub_ps(self.data.simd, rhs.data.simd),
                }
            },
        }
    }
}

impl<const NUM: usize> Mul for BasicVector<NUM> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        BasicVector {
            data: unsafe {
                F32x4 {
                    simd: _mm_mul_ps(self.data.simd, rhs.data.simd),
                }
            },
        }
    }
}

impl<const NUM: usize> Div for BasicVector<NUM> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        BasicVector {
            data: unsafe {
                F32x4 {
                    simd: _mm_div_ps(self.data.simd, rhs.data.simd),
                }
            },
        }
    }
}

impl<const NUM: usize> Div<BasicVector<NUM>> for f32 {
    type Output = BasicVector<NUM>;

    fn div(self, rhs: BasicVector<NUM>) -> Self::Output {
        if self == 1.0 {
            Self::Output {
                data: unsafe {
                    F32x4 {
                        simd: _mm_rcp_ps(rhs.data.simd),
                    }
                },
            }
        } else {
            let a = unsafe { _mm_set1_ps(self) };
            let b = unsafe { _mm_rcp_ps(rhs.data.simd) };
            BasicVector {
                data: unsafe {
                    F32x4 {
                        simd: _mm_mul_ps(a, b),
                    }
                },
            }
        }
    }
}

impl<const NUM: usize> Mul<BasicVector<NUM>> for f32 {
    type Output = BasicVector<NUM>;

    fn mul(self, rhs: BasicVector<NUM>) -> Self::Output {
        let a = unsafe { _mm_set1_ps(self) };
        BasicVector {
            data: unsafe {
                F32x4 {
                    simd: _mm_mul_ps(a, rhs.data.simd),
                }
            },
        }
    }
}

impl<const NUM: usize> Mul<f32> for BasicVector<NUM> {
    type Output = BasicVector<NUM>;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

pub type Vec4 = BasicVector<4>;
pub type Vec3 = BasicVector<3>;
pub type Vec2 = BasicVector<2>;

impl Vec4 {
    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            data: unsafe {
                F32x4 {
                    simd: _mm_set_ps(x, y, z, w),
                }
            },
        }
    }

    pub fn x(&self) -> f32 {
        unsafe { self.data.data.0 }
    }
    pub fn y(&self) -> f32 {
        unsafe { self.data.data.1 }
    }
    pub fn z(&self) -> f32 {
        unsafe { self.data.data.2 }
    }
    pub fn w(&self) -> f32 {
        unsafe { self.data.data.3 }
    }
}

impl Vec3 {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: unsafe {
                F32x4 {
                    simd: _mm_set_ps(x, y, z, 0.0),
                }
            },
        }
    }

    pub fn x(&self) -> f32 {
        unsafe { self.data.data.0 }
    }
    pub fn y(&self) -> f32 {
        unsafe { self.data.data.1 }
    }
    pub fn z(&self) -> f32 {
        unsafe { self.data.data.2 }
    }
}

impl Vec2 {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self {
            data: unsafe {
                F32x4 {
                    simd: _mm_set_ps(x, y, 0.0, 0.0),
                }
            },
        }
    }

    pub fn x(&self) -> f32 {
        unsafe { self.data.data.0 }
    }
    pub fn y(&self) -> f32 {
        unsafe { self.data.data.1 }
    }
}
