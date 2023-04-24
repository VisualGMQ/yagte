use raylib::prelude::*;
use visual_debugger::ent3d::*;

fn draw_mesh(mode: &mut RaylibMode3D<RaylibDrawHandle>, display_data: &FaceDisplayData) {
    for indices in display_data.indices.chunks(3) {
        for i in 0..indices.len() {
            let p1 = &display_data.vertices[indices[i] as usize];
            let p2 = &display_data.vertices[indices[(i + 1) % 3] as usize];
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
    println!("end");
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 720)
        .title("Hello, World")
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

    let polygon = geometric::geom3d::Polygon {
        points: vec![
            ::math::matrix::Vec3::from_xyz(1.0, 1.0, 1.0),
            ::math::matrix::Vec3::from_xyz(0.0, 1.0, 2.0),
            ::math::matrix::Vec3::from_xyz(-1.0, 1.0, 1.0),
            ::math::matrix::Vec3::from_xyz(-0.5, 1.0, -0.5),
            ::math::matrix::Vec3::from_xyz(0.5, 1.0, -0.5),
        ],
    };

    let display_data = plane_to_display_data(
        &polygon,
        ::math::matrix::Vec4::from_xyzw(0.0, 1.0, 1.0, 1.0),
    )
    .unwrap();

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

        draw_mesh(&mut mode, &display_data);
    }
}
