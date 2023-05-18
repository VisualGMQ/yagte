use crate::geom_common::*;
use math::{matrix::*, precision::real};

pub fn is_circular_contain_pt<const DIM: usize>(c: &Circular<DIM>, pt: &Vector<real, DIM>) -> bool {
    (*pt - c.center).length_sqrd() <= c.radius * c.radius
}