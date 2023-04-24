use geometric::geom3d::*;
use math::matrix::*;

pub struct FaceDisplayData {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub color: Vec4,
}

pub struct LineDisplayData {
    pub vertices: Vec<Vec3>,
    pub color: Vec4,
}

pub fn plane_to_display_data(polygon: &Polygon, color: Vec4) -> Result<FaceDisplayData, &str> {
    if polygon.points.len() < 3 {
        return Err("invalid polygon");
    }

    let vertices = polygon.points.clone();
    let normal = (vertices[1] - vertices[0])
        .cross(&(vertices[2] - vertices[1]))
        .normalize();
    let mut indices: Vec<u32> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new();

    for i in 1..vertices.len() as u32 - 1 {
        indices.extend([0, i, i + 1].iter());
        normals.push(normal);
    }

    Ok(FaceDisplayData {
        vertices,
        normals,
        indices,
        color,
    })
}

pub fn conic_curve_to_display_data(conic: &ConicArc, color: Vec4) -> LineDisplayData {
    match &conic.conic {
        Conic::Ellipse(e) => ellipse_curve_to_display_data(e, conic.range, color),
        Conic::Hyperbola(h) => hyperbola_curve_to_display_data(h, conic.range, color),
        Conic::Parabola(p) => parabola_curve_to_display_data(p, conic.range, color),
    }
}

fn ellipse_curve_to_display_data(
    ellipse: &Ellipse,
    range: (f32, f32),
    color: Vec4,
) -> LineDisplayData {
    const DEG_STEP: f32 = 0.01;
    let range = if range.0 > range.1 {
        (range.1, range.0)
    } else {
        range
    };

    let mut deg = range.0;
    let mut vertices: Vec<Vec3> = Vec::new();
    while deg < range.1 {
        let v = Vec3::from_xyz(ellipse.a * deg.cos(), 0.0, ellipse.b * deg.sin());
        vertices.push(ellipse.get_coord().transform(v));
        deg += DEG_STEP;
    }
    let v = Vec3::from_xyz(ellipse.a * range.1.cos(), 0.0, ellipse.b * range.1.sin());
    vertices.push(ellipse.get_coord().transform(v));

    LineDisplayData { vertices, color }
}

fn hyperbola_curve_to_display_data(
    hyperbola: &Hyperbola,
    range: (f32, f32),
    color: Vec4,
) -> LineDisplayData {
    const DEG_STEP: f32 = 0.01;
    let range = if range.0 > range.1 {
        (range.1, range.0)
    } else {
        range
    };

    let mut deg = range.0;
    let mut vertices: Vec<Vec3> = Vec::new();
    while deg < range.1 {
        let v = Vec3::from_xyz(hyperbola.a * 1.0 / deg.cos(), 0.0, hyperbola.b * deg.tan());
        vertices.push(hyperbola.get_coord().transform(v));
        deg += DEG_STEP;
    }
    let v = Vec3::from_xyz(
        hyperbola.a * 1.0 / range.1.cos(),
        0.0,
        hyperbola.b * range.1.tan(),
    );
    vertices.push(hyperbola.get_coord().transform(v));

    LineDisplayData { vertices, color }
}

fn parabola_curve_to_display_data(
    parabola: &Parabola,
    range: (f32, f32),
    color: Vec4,
) -> LineDisplayData {
    const DEG_STEP: f32 = 0.01;
    let range = if range.0 > range.1 {
        (range.1, range.0)
    } else {
        range
    };

    let mut deg = range.0;
    let mut vertices: Vec<Vec3> = Vec::new();
    while deg < range.1 {
        let v = Vec3::from_xyz(
            2.0 * parabola.p * 1.0 / (deg.tan() * deg.tan()),
            0.0,
            2.0 * parabola.p / deg.tan(),
        );
        vertices.push(parabola.get_coord().transform(v));
        deg += DEG_STEP;
    }
    let v = Vec3::from_xyz(
        2.0 * parabola.p * 1.0 / (range.1.tan() * range.1.tan()),
        0.0,
        2.0 * parabola.p / range.1.tan(),
    );
    vertices.push(parabola.get_coord().transform(v));

    LineDisplayData { vertices, color }
}


pub fn cylinder_to_display_data(cylinder: &Cylinder, color: Vec4) -> FaceDisplayData {
    todo!()
}

pub fn cone_to_display_data(cone: &Cone, color: Vec4) -> FaceDisplayData {
    todo!()
}

pub fn truncatedcone_to_display_data(cone: &TruncatedCone, color: Vec4) -> FaceDisplayData {
    todo!()
}

pub fn cylinderlike_to_display_data(like: &CylinderLike, color: Vec4) -> FaceDisplayData {
    todo!()
}
