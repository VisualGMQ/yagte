use crate::geom_common::*;
use math::{matrix::*, precision::Real};

pub fn is_circular_contain_pt<const DIM: usize>(c: &Circular<DIM>, pt: &Vector<Real, DIM>) -> bool {
    (*pt - c.center).length_sqrd() <= c.radius * c.radius
}
