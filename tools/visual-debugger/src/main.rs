use geometric::geom3d::*;
use raylib::prelude::*;
use graphics::mesh_generate::*;

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
                Vector3::new(p1.x(), p1.y(), p1.z()),
                Vector3::new(p2.x(), p2.y(), p2.z()),
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
        dir: ::math::matrix::Vec3::z_axis(),
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
        dir: ::math::matrix::Vec3::z_axis(),
        height: 1.0,
    };

    let cylinder_display_data = cylinder_to_display_data(
        &cylinder,
        ::math::matrix::Vec4::from_xyzw(0.0, 1.0, 0.0, 1.0),
        100,
    );

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
        draw_mesh(&mut mode, ::math::matrix::Vec3::zeros(), &cone_display_data);
        draw_mesh(
            &mut mode,
            ::math::matrix::Vec3::from_xyz(2.0, 0.0, 0.0),
            &cylinder_display_data,
        );
        draw_mesh(
            &mut mode,
            ::math::matrix::Vec3::from_xyz(4.0, 0.0, 0.0),
            &truncated_cone_display_data,
        );
    }
}
