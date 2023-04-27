use crate::geom3d::*;

pub fn planes_intersect(p1: &Plane, p2: &Plane) -> Line {
    let dir = p1.normal.cross(&p2.normal);
    let s1 = p1.normal.dot(&p1.pt);
    let s2 = p1.normal.dot(&p1.pt);
    let dot = p1.normal.dot(&p2.normal);
    let base = dot * dot - 1.0;
    let a = (s2 * dot - s1) / base;
    let b = (s1 * dot - s2) / base;

    Line::new(p1.normal * a + p2.normal * b, dir)
}
