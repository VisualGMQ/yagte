/// [Cohen-Sutherland Algorithm](https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm)
pub mod cohen_sutherland {
    use math::matrix::*;

    const INSIDE: u8 = 0;
    const LEFT: u8 = 1;
    const RIGHT: u8 = 2;
    const BOTTOM: u8 = 4;
    const TOP: u8 = 8;

    pub fn compute_outcode(p: &Vec2, min: &Vec2, max: &Vec2) -> u8 {
        (if p.x() < min.x() {
            LEFT
        } else if p.x() > max.x() {
            RIGHT
        } else {
            INSIDE
        } | if p.y() < min.y() {
            BOTTOM
        } else if p.y() > max.y() {
            TOP
        } else {
            INSIDE
        })
    }

    pub fn cohen_sutherland_line_clip(
        p1: &Vec2,
        p2: &Vec2,
        rect_min: &Vec2,
        rect_max: &Vec2,
    ) -> Option<(Vec2, Vec2)> {
        let mut pt1 = *p1;
        let mut pt2 = *p2;

        let mut outcode1 = compute_outcode(&pt1, rect_min, rect_max);
        let mut outcode2 = compute_outcode(&pt2, rect_min, rect_max);

        loop {
            if outcode1 & outcode2 != 0 {
                return None;
            } else if outcode1 | outcode2 == 0 {
                return Some((pt1, pt2));
            }

            let mut p = Vec2::zeros();

            let outcode = if outcode2 > outcode1 {
                outcode2
            } else {
                outcode1
            };

            if outcode & TOP != 0 {
                p[0] =
                    p1.x() + (pt2.x() - pt1.x()) * (rect_max.y() - pt1.y()) / (pt2.y() - pt1.y());
                p[1] = rect_max.y();
            } else if outcode & BOTTOM != 0 {
                p[0] =
                    p1.x() + (pt2.x() - pt1.x()) * (rect_min.y() - pt1.y()) / (pt2.y() - pt1.y());
                p[1] = rect_min.y();
            } else if outcode & RIGHT != 0 {
                p[1] =
                    pt1.y() + (pt2.y() - pt1.y()) * (rect_max.x() - pt1.x()) / (pt2.x() - pt1.x());
                p[0] = rect_max.x();
            } else if outcode & LEFT != 0 {
                p[1] =
                    pt1.y() + (pt2.y() - pt1.y()) * (rect_min.x() - pt1.x()) / (pt2.x() - pt1.x());
                p[0] = rect_min.x();
            }

            if outcode == outcode1 {
                pt1 = p;
                outcode1 = compute_outcode(&pt1, rect_min, rect_max);
            } else {
                pt2 = p;
                outcode2 = compute_outcode(&pt2, rect_min, rect_max);
            }
        }
    }
}

#[inline]
pub fn approx_equal(a: f32, b: f32, decimal_places: u8) -> bool {
    let factor = 10.0f32.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}
