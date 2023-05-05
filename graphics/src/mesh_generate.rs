use std::f32::consts::PI;

use geometric::{geom2d::Circle, geom3d::*};
use math::{coord::Cartesian3D, matrix::*};

pub struct FaceDisplayData {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u32>,
    pub color: Vec4,
}

pub struct LineStripDisplayData {
    pub vertices: Vec<Vec3>,
    pub color: Vec4,
}

pub fn polygon_to_display_data(polygon: &Polygon, color: Vec4) -> Result<FaceDisplayData, &str> {
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

pub fn polyline_to_display_data(polyline: &Vec<Vec3>, color: Vec4) -> Result<LineStripDisplayData, &str> {
    if polyline.len() < 2 {
        return Err("invalid polygon");
    }

    Ok(LineStripDisplayData {
        vertices: polyline.clone(),
        color,
    })
}

pub fn conic_curve_to_display_data(conic: &ConicArc, color: Vec4) -> LineStripDisplayData {
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
) -> LineStripDisplayData {
    const DEG_STEP: f32 = 0.01;
    let range = if range.0 > range.1 {
        (range.1, range.0)
    } else {
        range
    };

    let mut deg = range.0;
    let mut vertices: Vec<Vec3> = Vec::new();
    while deg < range.1 {
        let v = Vec3::from_xyz(ellipse.a * deg.cos(), ellipse.b * deg.sin(), 0.0);
        vertices.push(ellipse.get_coord().transform(v));
        deg += DEG_STEP;
    }
    let v = Vec3::from_xyz(ellipse.a * range.1.cos(), ellipse.b * range.1.sin(), 0.0);
    vertices.push(ellipse.get_coord().transform(v));

    LineStripDisplayData { vertices, color }
}

fn hyperbola_curve_to_display_data(
    hyperbola: &Hyperbola,
    range: (f32, f32),
    color: Vec4,
) -> LineStripDisplayData {
    const DEG_STEP: f32 = 0.01;
    let range = if range.0 > range.1 {
        (range.1, range.0)
    } else {
        range
    };

    let mut deg = range.0;
    let mut vertices: Vec<Vec3> = Vec::new();
    while deg < range.1 {
        let v = Vec3::from_xyz(hyperbola.a * 1.0 / deg.cos(), hyperbola.b * deg.tan(), 0.0);
        vertices.push(hyperbola.get_coord().transform(v));
        deg += DEG_STEP;
    }
    let v = Vec3::from_xyz(
        hyperbola.a * 1.0 / range.1.cos(),
        hyperbola.b * range.1.tan(),
        0.0,
    );
    vertices.push(hyperbola.get_coord().transform(v));

    LineStripDisplayData { vertices, color }
}

fn parabola_curve_to_display_data(
    parabola: &Parabola,
    range: (f32, f32),
    color: Vec4,
) -> LineStripDisplayData {
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
            2.0 * parabola.p / deg.tan(),
            0.0,
        );
        vertices.push(parabola.get_coord().transform(v));
        deg += DEG_STEP;
    }
    let v = Vec3::from_xyz(
        2.0 * parabola.p * 1.0 / (range.1.tan() * range.1.tan()),
        2.0 * parabola.p / range.1.tan(),
        0.0,
    );
    vertices.push(parabola.get_coord().transform(v));

    LineStripDisplayData { vertices, color }
}

pub fn cylinder_to_display_data(cylinder: &Cylinder, color: Vec4, slice: u32) -> FaceDisplayData {
    truncatedcone_to_display_data(
        &TruncatedCone {
            bottom: cylinder.bottom,
            bottom_radius: cylinder.radius,
            top_radius: cylinder.radius,
            dir: cylinder.dir,
            height: cylinder.height,
        },
        color,
        slice,
    )
}

pub fn cone_to_display_data(cone: &Cone, color: Vec4, slice: u32) -> FaceDisplayData {
    let mut circle_data = circle_to_display_data(
        &Circle {
            center: Vec2::from_xy(cone.bottom.x(), cone.bottom.z()),
            radius: cone.bottom_radius,
        },
        color,
        slice,
    );
    let top = cone.bottom + cone.dir * cone.height;

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    vertices.append(&mut circle_data.vertices);
    vertices.push(top);
    indices.append(&mut circle_data.indices);

    let last_idx = vertices.len() as u32 - 1;
    for i in 0..slice {
        indices.extend([last_idx, i, (i + 1) % slice].iter());
    }

    // TODO: normals not calculated
    FaceDisplayData {
        vertices,
        normals: vec![],
        indices,
        color,
    }
}

pub fn truncatedcone_to_display_data(
    cone: &TruncatedCone,
    color: Vec4,
    slice: u32,
) -> FaceDisplayData {
    let mut bottom = origin_circle_to_display_data(cone.bottom_radius, color, slice);
    let top = origin_circle_to_display_data(cone.top_radius, color, slice);

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    vertices.append(&mut bottom.vertices);
    indices.append(&mut bottom.indices);
    vertices.append(
        &mut top
            .vertices
            .iter()
            .map(|v| *v + Vec3::z_axis() * cone.height)
            .collect(),
    );
    indices.append(&mut top.indices.iter().map(|i| *i + slice).collect());

    for i in 0..slice {
        indices.extend([i, i + slice, (i + 1) % slice].iter());
        indices.extend([i + 1, i + slice, (i + slice + 1) % (slice * 2)].iter());
    }

    let x_axis = (geometric::misc::get_arbitrary_from_plane(&Vec3::z_axis(), &cone.bottom)
        - cone.bottom)
        .normalize();
    let y_axis = cone.dir.cross(&x_axis).normalize();
    let x_axis = y_axis.cross(&cone.dir).normalize();

    let cart = Cartesian3D::new(x_axis, y_axis, cone.dir, cone.bottom);
    for v in &mut vertices {
        *v = cart.transform(*v);
    }

    FaceDisplayData {
        vertices,
        normals: vec![],
        indices,
        color,
    }
}

pub fn cylinderlike_to_display_data(
    like: &CylinderLike,
    color: Vec4,
    slice: u32,
) -> FaceDisplayData {
    match &like {
        CylinderLike::Cylinder(c) => cylinder_to_display_data(c, color, slice),
        CylinderLike::Cone(c) => cone_to_display_data(c, color, slice),
        CylinderLike::TruncatedCone(c) => truncatedcone_to_display_data(c, color, slice),
    }
}

pub fn circle_to_display_data(circle: &Circle, color: Vec4, slice: u32) -> FaceDisplayData {
    let deg_step = 2.0 * PI / slice as f32;
    let polygon = Polygon {
        points: (0..slice)
            .map(|i| {
                let i = i as f32;
                let deg = i * deg_step;
                Vec3::from(circle.center)
                    + Vec3::from_xyz(deg.cos() * circle.radius, deg.sin() * circle.radius, 0.0)
            })
            .collect(),
    };

    polygon_to_display_data(&polygon, color).unwrap()
}

pub fn circle_arc_to_display_data(arc: &CircleArc, color: Vec4, slice: u32) -> LineStripDisplayData {
    let deg_step = (arc.range.1 - arc.range.0) / slice as f32;

    let z_axis = arc.x_axis.cross(&arc.norm);
    let cart = Cartesian3D::new(arc.x_axis, arc.norm, z_axis, arc.center);

    let points: Vec<Vec3> = (0..slice).map(|i| {
        let deg = arc.range.0 + i as f32 * deg_step;
        let point = Vec3::from_xyz(deg.sin() * arc.radius, 0.0, deg.cos() * arc.radius);
        cart.transform(point)
    }).collect();

    polyline_to_display_data(&points, color).unwrap()
}

pub fn origin_circle_to_display_data(radius: f32, color: Vec4, slice: u32) -> FaceDisplayData {
    let deg_step = 2.0 * PI / slice as f32;
    let polygon = Polygon {
        points: (0..slice)
            .map(|i| {
                let i = i as f32;
                let deg = i * deg_step;
                Vec3::from_xyz(deg.cos() * radius, deg.sin() * radius, 0.0)
            })
            .collect(),
    };

    polygon_to_display_data(&polygon, color).unwrap()
}

pub fn conic_arc_to_display_data(arc: &ConicArc, color: Vec4, slice: u32) -> LineStripDisplayData {
    let deg_step = (arc.range.1 - arc.range.0) / slice as f32;

    let x_axis = arc.x_axis();
    let normal = arc.normal();
    let z_axis = x_axis.cross(&normal);
    let cart = Cartesian3D::new(x_axis, normal, z_axis, arc.center());

    let points: Vec<Vec3> = (0..slice).map(|i| {
        let deg = arc.range.0 + i as f32 * deg_step;
        let point = match arc.conic {
            Conic::Ellipse(e) => {
                Vec3::from_xyz(e.a * deg.cos(), e.b * deg.sin(), 0.0)
            },
            Conic::Hyperbola(h) => {
                Vec3::from_xyz(h.a / deg.cos(), h.b * deg.tan(), 0.0)
            }
            Conic::Parabola(p) => {
                let t = deg;
                Vec3::from_xyz(2.0 * p.p * t * t, 2.0 * p.p * t, 0.0)
            }
        };
        cart.transform(point)
    }).collect();

    polyline_to_display_data(&points, color).unwrap()
}