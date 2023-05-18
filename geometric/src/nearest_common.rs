use crate::geom_common::*;
use math::{matrix::*, precision::real};

pub fn pt2line_param<const DIM: usize>(pt: &Vector<real, DIM>, start: &Vector<real, DIM>, dir: &Vector<real, DIM>) -> real {
    (*pt - *start).dot(&dir)
}

pub fn pt2line<const DIM: usize>(pt: &Vector<real, DIM>, start: &Vector<real, DIM>, dir: &Vector<real, DIM>) -> Vector<real, DIM> {
    let t = pt2line_param(pt, start, dir);
    *start + *dir * t
}

pub fn pt2ray_param<const DIM: usize>(pt: &Vector<real, DIM>, ray: &Ray<DIM>) -> real {
    let t = pt2line_param(pt, &ray.start, &ray.dir);
    t.max(0.0)
}

pub fn pt2ray<const DIM: usize>(pt: &Vector<real, DIM>, ray: &Ray<DIM>) -> Vector<real, DIM> {
    let t = pt2ray_param(pt, ray);
    ray.start + ray.dir * t
}

pub fn pt2segment_param<const DIM: usize>(pt: &Vector<real, DIM>, seg: &Segment<DIM>) -> real {
    let t = pt2line_param(pt, &seg.start, &seg.dir);
    t.clamp(0.0, seg.len)
}

pub fn pt2segment<const DIM: usize>(pt: &Vector<real, DIM>, seg: &Segment<DIM>) -> Vector<real, DIM> {
    let t = pt2segment_param(pt, seg);
    seg.start + seg.dir * t
}
