mod arithmetic;
pub mod cg;
pub mod coord;
pub mod matrix;
pub mod precision;

#[cfg(target_arch = "x86_64")]
pub mod simd_matrix;
