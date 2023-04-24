mod arithmetic;
pub mod cg;
pub mod coord;
pub mod matrix;

#[cfg(target_arch = "x86_64")]
pub mod simd_matrix;
