use crate::geom_common::*;

pub fn is_circular_intersect<const DIM: usize>(c1: &Circular<DIM>, c2: &Circular<DIM>) -> bool {
    let len_sqrd = (c2.center - c1.center).length_sqrd();
    len_sqrd >= (c1.radius - c2.radius) * (c1.radius - c2.radius)
        && len_sqrd <= (c1.radius + c2.radius) * (c1.radius + c2.radius)
}
