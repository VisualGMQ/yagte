use ::math::{
    matrix::{Vec3, Vec4},
    precision::Real,
};
use geometric::geom3d::*;
use graphics::mesh_generate::*;
use raylib::prelude::*;

fn draw_mesh(
    mode: &mut RaylibMode3D<RaylibDrawHandle>,
    position: ::math::matrix::Vec3,
    display_data: &FaceDisplayData,
) {
    for indices in display_data.indices.chunks(3) {
        for i in 0..indices.len() {
            let p1 = display_data.vertices[indices[i] as usize] + position;
            let p2 = display_data.vertices[indices[(i + 1) % 3] as usize] + position;
            mode.draw_line_3D(
                Vector3::new(p1.x() as f32, p1.y() as f32, p1.z() as f32),
                Vector3::new(p2.x() as f32, p2.y() as f32, p2.z() as f32),
                Color::new(
                    (display_data.color.x() * 255.0).clamp(0.0, 255.0) as u8,
                    (display_data.color.y() * 255.0).clamp(0.0, 255.0) as u8,
                    (display_data.color.z() * 255.0).clamp(0.0, 255.0) as u8,
                    (display_data.color.w() * 255.0).clamp(0.0, 255.0) as u8,
                ),
            );
        }
    }
}

fn draw_polyline(
    mode: &mut RaylibMode3D<RaylibDrawHandle>,
    position: ::math::matrix::Vec3,
    line_data: &LineStripDisplayData,
) {
    for i in 0..line_data.vertices.len() - 1 {
        let p1 = line_data.vertices[i] + position;
        let p2 = line_data.vertices[i + 1] + position;
        mode.draw_line_3D(
            Vector3::new(p1.x() as f32, p1.y() as f32, p1.z() as f32),
            Vector3::new(p2.x() as f32, p2.y() as f32, p2.z() as f32),
            Color::new(
                (line_data.color.x() * 255.0).clamp(0.0, 255.0) as u8,
                (line_data.color.y() * 255.0).clamp(0.0, 255.0) as u8,
                (line_data.color.z() * 255.0).clamp(0.0, 255.0) as u8,
                (line_data.color.w() * 255.0).clamp(0.0, 255.0) as u8,
            ),
        );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 720)
        .title("display")
        .msaa_4x()
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 2.0, 4.0),
        Vector3::new(0.0, 1.8, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );
    rl.set_camera_mode(&camera, CameraMode::CAMERA_FIRST_PERSON);
    rl.set_target_fps(60);

    let cone = Cone {
        bottom: ::math::matrix::Vec3::zeros(),
        bottom_radius: 1.0,
        dir: ::math::matrix::Vec3::z_axis(),
        height: 1.0,
    };

    let cone_display_data = cone_to_display_data(
        &cone,
        ::math::matrix::Vec4::from_xyzw(1.0, 0.0, 0.0, 1.0),
        100,
    );

    let truncated_cone = TruncatedCone {
        bottom: ::math::matrix::Vec3::zeros(),
        bottom_radius: 1.0,
        top_radius: 0.5,
        dir: ::math::matrix::Vec3::from_xyz(0.0, 0.0, 1.0),
        height: 1.0,
    };

    let truncated_cone_display_data = truncatedcone_to_display_data(
        &truncated_cone,
        ::math::matrix::Vec4::from_xyzw(0.0, 0.0, 1.0, 1.0),
        100,
    );

    let cylinder = Cylinder {
        bottom: ::math::matrix::Vec3::zeros(),
        radius: 1.0,
        dir: ::math::matrix::Vec3::from_xyz(0.0, 1.0, 1.0),
        height: 1.0,
    };

    let cylinder_display_data = cylinder_to_display_data(
        &cylinder,
        ::math::matrix::Vec4::from_xyzw(0.0, 1.0, 0.0, 1.0),
        100,
    );

    let circle_arc = circle_arc_to_display_data(
        &CircleArc {
            radius: 2.0,
            center: Vec3::zeros(),
            norm: Vec3::y_axis(),
            x_axis: Vec3::x_axis(),
            range: ((45.0 as Real).to_radians(), (-90.0 as Real).to_radians()),
        },
        ::math::matrix::Vec4::from_xyzw(0.0, 1.0, 1.0, 1.0),
        100,
    );

    let ellipse_arc = ConicArc {
        conic: Conic::Parabola(Parabola {
            x_axis: Vec3::x_axis(),
            normal: Vec3::y_axis(),
            p: 2.0,
            position: Vec3::zeros(),
        }),
        range: ((45.0 as Real).to_radians(), (-45.0 as Real).to_radians()),
    };
    let ellipse_arc =
        conic_arc_to_display_data(&ellipse_arc, Vec4::from_xyzw(1.0, 1.0, 0.0, 1.0), 100);

    let ellipse_arc2 = ConicArc {
        conic: Conic::Ellipse(Ellipse {
            x_axis: Vec3::x_axis(),
            normal: Vec3::y_axis(),
            a: 1.0,
            b: 1.0,
            position: Vec3::zeros(),
        }),
        range: ((45.0 as Real).to_radians(), (-45.0 as Real).to_radians()),
    };
    let ellipse_arc2 =
        conic_arc_to_display_data(&ellipse_arc2, Vec4::from_xyzw(0.0, 1.0, 1.0, 1.0), 100);

    let ellipse_arc3 = ConicArcInPolar::new(
        2.0,
        1.3,
        Vec3::zeros(),
        Vec3::x_axis(),
        Vec3::y_axis(),
        ((45.0 as Real).to_radians(), (-45.0 as Real).to_radians()),
    );
    let ellipse_arc3 =
        polar_conic_arc_to_display_data(&ellipse_arc3, Vec4::from_xyzw(0.5, 0.3, 1.0, 1.0), 100);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);
        let mut d = rl.begin_drawing(&thread);
        let mut mode = d.begin_mode3D(camera);

        mode.clear_background(Color::DARKGRAY);
        mode.draw_grid(100, 1.0);
        mode.draw_line_3D(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(100.0, 0.1, 0.0),
            Color::RED,
        );
        mode.draw_line_3D(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 100.0, 0.0),
            Color::GREEN,
        );
        mode.draw_line_3D(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.1, 100.0),
            Color::BLUE,
        );

        // draw_mesh(&mut mode, &display_data);
        // draw_mesh(&mut mode, ::math::matrix::Vec3::zeros(), &cone_display_data);
        // draw_mesh(
        //     &mut mode,
        //     ::math::matrix::Vec3::from_xyz(2.0, 0.0, 0.0),
        //     &cylinder_display_data,
        // );
        // draw_mesh(
        //     &mut mode,
        //     ::math::matrix::Vec3::from_xyz(6.0, 0.0, 0.0),
        //     &truncated_cone_display_data,
        // );
        // draw_polyline(&mut mode, Vec3::zeros(), &circle_arc);
        // draw_polyline(&mut mode, Vec3::zeros(), &ellipse_arc);
        // draw_polyline(&mut mode, Vec3::zeros(), &ellipse_arc2);
        draw_polyline(&mut mode, Vec3::zeros(), &ellipse_arc3);
    }
}
